// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RME Fireface UCX (USB 2a39:3fc9) in its vendor mode: audio both ways on the isochronous
 * interface 0, as an ALSA card, and the MIDI ports of interface 2 on the same card. Interface 2
 * is standard USB-MIDI (class 01/03, "Port 1" and "Port 2": the unit's two MIDI I/Os, four DIN
 * sockets on its breakout cable), but the unit has no AudioControl interface, so snd-usb-audio
 * doesn't take it; snd-usbmidi-lib drives it here, as snd-ua101 does its MIDI interface. The
 * mixer and meters on interface 1 stay free for userspace, so `ucx-stream meters` and a mixer app
 * run next to it.
 *
 * The protocol is in re/NOTES.md and crates/ucx-protocol; crates/ucx-stream does the same from
 * userspace and is the reference this follows (linux/DRIVER_PLAN.md).
 *
 * One packet per microframe each way, float32 LE samples in DSP channel order (analog 1-8,
 * SPDIF L/R, ADAT 1-8: 18 channels at 1x, 14 at 2x, 12 at 4x):
 *
 * - IN (0x8f): a header word (frames in bits 0-4, the unit's position of the first frame modulo
 *   3072 in bits 5-16, a timestamp above), then the frames planar.
 * - OUT (0x0e): frames interleaved, each led by a word with its position modulo 3072. The unit
 *   plays a frame at that position of its own frame clock, out of a 128-frame ring, so a
 *   packet's first frame has to be about `lead` frames after the unit's position in the
 *   microframe the packet goes out in. Positions it gets no frames for play silence.
 *
 * Both streams run whenever either PCM direction is in use, on one microframe timeline taken
 * from the URBs' start_frame: they go in without URB_ISO_ASAP, so an xHCI with Contiguous Frame
 * ID support (HCCPARAMS1 bit 11, as Intel's have) gives every TD an explicit frame and reports
 * it, contiguous while the queue doesn't run dry (xhci_queue_isoc_tx_prepare; a URB queued too
 * close to the current frame is moved a frame later, and start_frame says so). Without CFC
 * start_frame is a guess; /proc/asound/cardN/ucx compares it with completion times. IN headers
 * give the unit's position per microframe. Each OUT packet's first lead word is the position predicted for its
 * microframe plus the lead, and its frame count whatever keeps the next packet on target, so
 * the lead words run on without gaps and follow the unit's clock (`build_out` in
 * crates/ucx-stream/src/engine.rs).
 */

#include <linux/module.h>
#include <linux/slab.h>
#include <linux/usb.h>
#include <linux/usb/audio.h>
#include <sound/core.h>
#include <sound/info.h>
#include <sound/initval.h>
#include <sound/pcm.h>
#include <sound/pcm_params.h>

#include "usbmidi.h"

#define UCX_VENDOR		0x2a39
#define UCX_PRODUCT		0x3fc9
#define UCX_EP_OUT		0x0e
#define UCX_EP_IN		0x8f
#define UCX_IF_MIDI		2

/* vendor requests on endpoint 0 (crates/ucx-protocol/src/control.rs) */
#define UCX_REQ_CONTROL		16	/* masked control register write: wValue bits, wIndex mask */
#define UCX_REQ_STATUS		17
#define UCX_REQ_MUTE		19
#define UCX_REQ_UNMUTE		20
#define UCX_REQ_FIRMWARE	28
#define UCX_REQ_STREAM		29

#define UCX_POSITION_WRAP	3072
#define UCX_MAX_URBS		32
#define UCX_MAX_PACKETS		64
#define UCX_UFRAME_NS		125000
/*
 * IN packets in a row that follow on from each other before OUT starts: the first ones after
 * SET_INTERFACE can be stale (position 0, then a jump of 1200 to 1900 frames).
 */
#define UCX_STEADY_PACKETS	400
/* The most capture frames filled with silence for IN packets that went missing. */
#define UCX_MAX_FILL		1024
/*
 * start_frame is only used modulo this, the microframes xHCI counts (it lets start_frame run past
 * that while a stream is contiguous), so the two streams stay apart by less than a second.
 */
#define UCX_UFRAME_MOD		16384

static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
static unsigned int packets = 8;
static unsigned int urbs = 4;
static unsigned int lead_frames = 60;
static bool midi = true;

module_param_array(index, int, NULL, 0444);
MODULE_PARM_DESC(index, "Index value for the Fireface UCX.");
module_param_array(id, charp, NULL, 0444);
MODULE_PARM_DESC(id, "ID string for the Fireface UCX.");
module_param_array(enable, bool, NULL, 0444);
MODULE_PARM_DESC(enable, "Enable the Fireface UCX.");
module_param(packets, uint, 0644);
MODULE_PARM_DESC(packets, "Microframes per URB, taken at stream start (default 8).");
module_param(urbs, uint, 0644);
MODULE_PARM_DESC(urbs, "URBs in flight per direction, taken at stream start (default 4).");
module_param_named(lead, lead_frames, uint, 0644);
MODULE_PARM_DESC(lead, "Frames from the unit's position to an OUT packet's first frame (default 60).");
module_param(midi, bool, 0444);
MODULE_PARM_DESC(midi, "Take the MIDI interface and put its ports on the card (default on).");

static const unsigned int ucx_rates[] = {
	32000, 44100, 48000, 64000, 88200, 96000, 128000, 176400, 192000,
};

static const struct snd_pcm_hw_constraint_list ucx_rate_list = {
	.count = ARRAY_SIZE(ucx_rates),
	.list = ucx_rates,
};

struct ucx;

struct ucx_urb {
	struct ucx *ucx;
	struct urb *urb;
	unsigned int bytes;
	u64 uframe;			/* the microframe of its first packet */
};

/* One PCM direction. */
struct ucx_pcm {
	struct snd_pcm_substream *substream;	/* (lock) while open */
	bool active;			/* between trigger start and stop */
	bool has_params;		/* (mutex) between hw_params and hw_free */
	unsigned int hw_ptr;		/* (lock) frames into the buffer */
	unsigned int period_pos;	/* (lock) */
	snd_pcm_sframes_t delay;	/* playback: frames sent that the unit hasn't played */
};

struct ucx_stats {
	u64 in_packets, in_errors, out_errors, jumps, in_gaps, out_gaps, resyncs;
	u64 filled, capture_resyncs, starts;
	int in_status, out_status;
	int in_start, out_start;	/* start_frame of the first URBs */
	/* earliest completion against each stream's microframes, ns (see ucx_proc_read) */
	s64 in_lag, out_lag;
	bool have_lag;
};

struct ucx {
	struct usb_device *dev;
	struct usb_interface *intf;
	struct usb_interface *midi_intf;	/* claimed, or NULL */
	struct list_head midi_list;		/* snd-usbmidi-lib's, one entry with MIDI */
	struct snd_card *card;
	struct snd_pcm *pcm;
	int card_index;
	u32 firmware;			/* as RME writes it, e.g. 03013100 */
	struct mutex mutex;		/* engine start and stop, users, rate */
	spinlock_t lock;		/* what the URB completions touch */
	bool disconnected;

	unsigned int users;		/* (mutex) PCM directions between hw_params and hw_free */
	unsigned int rate;		/* (mutex) their rate */

	/* the engine; set while it is stopped, under mutex */
	bool running;
	unsigned int rate_running, channels, max_frames, in_size, out_size, packets, nurbs;
	struct ucx_urb in[UCX_MAX_URBS], out[UCX_MAX_URBS];

	/* the rest under lock */
	bool streaming;			/* completions handle and resubmit URBs */
	bool broken;			/* a URB couldn't be resubmitted */
	u64 in_next, out_next;		/* the microframe of the next URB to submit; 0 = none yet */

	/* the unit's frame clock, from the IN headers */
	bool have_position;
	u64 pos_uframe;			/* the newest IN packet's microframe */
	s64 pos_after;			/* the unit's position just after its frames */
	unsigned int steady;		/* IN packets since the last jump */
	bool clock_ok;			/* steady long enough for OUT to start */

	bool out_locked;
	s64 lead_next;			/* the lead word of the next OUT frame */

	bool capture_started;
	s64 capture_pos;		/* the unit's position of the next capture frame */

	struct ucx_pcm playback, capture;
	struct ucx_stats stats;
};

static DEFINE_MUTEX(ucx_devices_mutex);
static bool ucx_used[SNDRV_CARDS];

/* 1x up to 48 kHz, 2x up to 96 kHz, 4x above. */
static unsigned int ucx_speed(unsigned int rate)
{
	return rate <= 48000 ? 1 : rate <= 96000 ? 2 : 4;
}

static unsigned int ucx_channels(unsigned int rate)
{
	return rate <= 48000 ? 18 : rate <= 96000 ? 14 : 12;
}

/* Control register bits 4-5: 32, 44.1 or 48 kHz family. */
static unsigned int ucx_base_rate(unsigned int rate)
{
	unsigned int base = rate / ucx_speed(rate);

	return base == 32000 ? 0 : base == 44100 ? 1 : 2;
}

static int ucx_write(struct ucx *ucx, u8 request, u16 value, u16 windex)
{
	int err = usb_control_msg(ucx->dev, usb_sndctrlpipe(ucx->dev, 0), request,
				  USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
				  value, windex, NULL, 0, 1000);

	if (err < 0)
		dev_err(&ucx->intf->dev, "request %u %04x/%04x: %d\n", request, value, windex, err);
	return err < 0 ? err : 0;
}

static int ucx_read(struct ucx *ucx, u8 request, u32 *word)
{
	__le32 v;
	int err = usb_control_msg_recv(ucx->dev, 0, request,
				       USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
				       0, 0, &v, sizeof(v), 1000, GFP_KERNEL);

	if (!err)
		*word = le32_to_cpu(v);
	return err;
}

static int ucx_mod(s64 x)
{
	s32 r;

	div_s64_rem(x, UCX_POSITION_WRAP, &r);
	return r < 0 ? r + UCX_POSITION_WRAP : r;
}

/* Frames the unit plays in n microframes, rounded. */
static s64 ucx_frames(const struct ucx *ucx, s64 n)
{
	s64 x = n * ucx->rate_running;

	return x >= 0 ? div_s64(x + 4000, 8000) : -div_s64(-x + 4000, 8000);
}

/* The unit's position at the end of microframe u, from the newest IN packet. */
static s64 ucx_predict(const struct ucx *ucx, u64 u)
{
	return ucx->pos_after + ucx_frames(ucx, (s64)(u - ucx->pos_uframe));
}

/* The microframe start_frame names, near `expect`. */
static u64 ucx_uframe(u64 expect, int start_frame)
{
	int d = (start_frame - (int)(expect % UCX_UFRAME_MOD)) & (UCX_UFRAME_MOD - 1);

	if (d >= UCX_UFRAME_MOD / 2)
		d -= UCX_UFRAME_MOD;
	return expect + d;
}

static bool ucx_urb_killed(int status)
{
	return status == -ENOENT || status == -ECONNRESET || status == -ESHUTDOWN ||
	       status == -ENODEV;
}

/* Under lock: stops whichever PCM directions run, as an xrun; they prepare again. */
static void ucx_xrun(struct ucx *ucx)
{
	if (ucx->playback.substream && READ_ONCE(ucx->playback.active))
		snd_pcm_stop_xrun(ucx->playback.substream);
	if (ucx->capture.substream && READ_ONCE(ucx->capture.active))
		snd_pcm_stop_xrun(ucx->capture.substream);
}

/* Under lock: a URB couldn't be resubmitted, so the engine has to start over. */
static void ucx_fail(struct ucx *ucx, int err)
{
	/* (gone, or its endpoint disabled while unbinding) */
	if (!ucx->broken && err != -ENODEV && err != -ESHUTDOWN && err != -ENOENT)
		dev_err(&ucx->intf->dev, "resubmitting a URB: %d\n", err);
	ucx->broken = true;
	ucx_xrun(ucx);
}

/*
 * Under lock: submits a URB and puts it on the microframe timeline, right after the one
 * submitted before it unless the queue ran dry (a gap). The first IN URB starts the timeline,
 * anywhere far from 0; the first OUT URB goes near it.
 */
static int ucx_submit(struct ucx *ucx, struct ucx_urb *u, bool out, bool first)
{
	u64 *next = out ? &ucx->out_next : &ucx->in_next;
	int err = usb_submit_urb(u->urb, GFP_ATOMIC);

	if (err)
		return err;
	if (first && !out) {
		u->uframe = (1ULL << 32) + (u->urb->start_frame & (UCX_UFRAME_MOD - 1));
	} else {
		u->uframe = ucx_uframe(first ? ucx->in[0].uframe : *next, u->urb->start_frame);
		if (!first && u->uframe != *next) {
			if (out)
				ucx->stats.out_gaps++;
			else
				ucx->stats.in_gaps++;
		}
	}
	*next = u->uframe + ucx->packets;
	return 0;
}

/* Under lock: the earliest completion against the end of a URB's microframes. */
static void ucx_lag(struct ucx *ucx, struct ucx_urb *u, bool out)
{
	s64 e = ktime_get_ns() - (s64)(u->uframe + ucx->packets) * UCX_UFRAME_NS;
	s64 *lag = out ? &ucx->stats.out_lag : &ucx->stats.in_lag;

	if (!ucx->stats.have_lag) {
		ucx->stats.in_lag = S64_MAX;
		ucx->stats.out_lag = S64_MAX;
		ucx->stats.have_lag = true;
	}
	*lag = min(*lag, e);
}

static void ucx_elapsed(struct ucx_pcm *p)
{
	if (p->substream && READ_ONCE(p->active))
		snd_pcm_period_elapsed(p->substream);
}

/* Under lock: one capture frame from planar src (frame f of `frames`), or silence. */
static void ucx_capture_frame(struct ucx *ucx, const __le32 *src, unsigned int frames,
			      unsigned int f, bool *elapsed)
{
	struct ucx_pcm *c = &ucx->capture;
	struct snd_pcm_runtime *rt = c->substream->runtime;
	unsigned int ch = ucx->channels, i;
	__le32 *dst = (__le32 *)rt->dma_area + c->hw_ptr * ch;

	for (i = 0; i < ch; i++)
		dst[i] = src ? src[i * frames + f] : 0;
	WRITE_ONCE(c->hw_ptr, c->hw_ptr + 1 == rt->buffer_size ? 0 : c->hw_ptr + 1);
	if (++c->period_pos >= rt->period_size) {
		c->period_pos = 0;
		*elapsed = true;
	}
}

/*
 * Under lock: an IN packet's frames, recorded at positions first .. first + frames. The stream
 * stays on the unit's clock: frames of packets that went missing are filled with silence.
 */
static void ucx_capture(struct ucx *ucx, s64 first, unsigned int frames, const __le32 *src,
			bool *elapsed)
{
	unsigned int f = 0;
	s64 gap;

	if (!ucx->capture_started) {
		ucx->capture_pos = first;
		ucx->capture_started = true;
	}
	gap = first - ucx->capture_pos;
	if (gap < 0) {
		if (-gap >= frames)
			return;
		f = -gap;
	} else if (gap > UCX_MAX_FILL) {
		ucx->stats.capture_resyncs++;
	} else {
		ucx->stats.filled += gap;
		while (gap--)
			ucx_capture_frame(ucx, NULL, 0, 0, elapsed);
	}
	for (; f < frames; f++)
		ucx_capture_frame(ucx, src, frames, f, elapsed);
	ucx->capture_pos = first + frames;
}

/* Under lock: the IN packet of microframe u. */
static void ucx_take_in(struct ucx *ucx, u64 u, const struct usb_iso_packet_descriptor *d,
			const u8 *buf, bool *elapsed)
{
	const __le32 *p = (const __le32 *)(buf + d->offset);
	unsigned int frames, position;
	s64 first;
	u32 word;

	if (d->status || d->actual_length < 4) {
		ucx->stats.in_errors++;
		ucx->stats.in_status = d->status;
		return;
	}
	word = le32_to_cpu(p[0]);
	frames = word & 0x1f;
	position = (word >> 5) & 0xfff;
	if (d->actual_length != 4 + frames * ucx->channels * 4) {
		ucx->stats.in_errors++;
		ucx->stats.in_status = -EMSGSIZE;
		return;
	}
	ucx->stats.in_packets++;

	if (!ucx->have_position) {
		first = position;
		ucx->have_position = true;
	} else {
		/* where this packet's first frame should be, after any missed microframes */
		s64 expect = ucx->pos_after + ucx_frames(ucx, (s64)(u - ucx->pos_uframe) - 1);
		int delta = ucx_mod(position - expect);

		if (delta >= UCX_POSITION_WRAP / 2)
			delta -= UCX_POSITION_WRAP;
		if (delta && (u == ucx->pos_uframe + 1 || abs(delta) > 2)) {
			ucx->stats.jumps++;
			ucx->steady = 0;
		}
		first = expect + delta;
	}
	ucx->pos_uframe = u;
	ucx->pos_after = first + frames;
	if (++ucx->steady >= UCX_STEADY_PACKETS)
		ucx->clock_ok = true;

	if (ucx->capture.substream && READ_ONCE(ucx->capture.active))
		ucx_capture(ucx, first, frames, p + 1, elapsed);
}

/* Under lock: the OUT packet for microframe u into dst; returns its length. */
static unsigned int ucx_out_packet(struct ucx *ucx, u64 u, s64 lead, __le32 *dst, bool *elapsed)
{
	struct ucx_pcm *p = &ucx->playback;
	struct snd_pcm_runtime *rt = NULL;
	unsigned int ch = ucx->channels, f, position;
	s64 target, n;

	if (!ucx->clock_ok)
		return 0;	/* positions without frames play silence */
	target = ucx_predict(ucx, u) + lead;
	if (!ucx->out_locked || abs(ucx->lead_next - target) > ucx->max_frames) {
		if (ucx->out_locked)
			ucx->stats.resyncs++;
		ucx->lead_next = target;
		ucx->out_locked = true;
	}
	n = clamp_t(s64, ucx_predict(ucx, u + 1) + lead - ucx->lead_next, 0, ucx->max_frames);
	if (p->substream && READ_ONCE(p->active))
		rt = p->substream->runtime;
	position = ucx_mod(ucx->lead_next);
	for (f = 0; f < n; f++) {
		*dst++ = cpu_to_le32((position + f) % UCX_POSITION_WRAP);
		if (rt) {
			memcpy(dst, rt->dma_area + frames_to_bytes(rt, p->hw_ptr), ch * 4);
			WRITE_ONCE(p->hw_ptr, p->hw_ptr + 1 == rt->buffer_size ? 0 : p->hw_ptr + 1);
			if (++p->period_pos >= rt->period_size) {
				p->period_pos = 0;
				*elapsed = true;
			}
		} else {
			memset(dst, 0, ch * 4);
		}
		dst += ch;
	}
	ucx->lead_next += n;
	return n * (ch + 1) * 4;
}

/* Under lock: fills an OUT URB for the microframes from out_next on. */
static void ucx_fill_out(struct ucx *ucx, struct ucx_urb *u, bool *elapsed)
{
	struct urb *urb = u->urb;
	s64 lead = READ_ONCE(lead_frames);
	unsigned int i;

	for (i = 0; i < ucx->packets; i++) {
		struct usb_iso_packet_descriptor *d = &urb->iso_frame_desc[i];

		d->offset = i * ucx->out_size;
		d->length = ucx_out_packet(ucx, ucx->out_next + i, lead,
					   urb->transfer_buffer + d->offset, elapsed);
	}
	if (ucx->out_locked)
		WRITE_ONCE(ucx->playback.delay, ucx->lead_next - ucx->pos_after);
}

static void ucx_in_complete(struct urb *urb)
{
	struct ucx_urb *u = urb->context;
	struct ucx *ucx = u->ucx;
	bool elapsed = false;
	unsigned long flags;
	int i, err;

	if (ucx_urb_killed(urb->status))
		return;
	spin_lock_irqsave(&ucx->lock, flags);
	if (!ucx->streaming)
		goto unlock;
	ucx_lag(ucx, u, false);
	for (i = 0; i < urb->number_of_packets; i++)
		ucx_take_in(ucx, u->uframe + i, &urb->iso_frame_desc[i], urb->transfer_buffer,
			    &elapsed);
	err = ucx_submit(ucx, u, false, false);
	if (err)
		ucx_fail(ucx, err);
	if (elapsed)
		ucx_elapsed(&ucx->capture);
unlock:
	spin_unlock_irqrestore(&ucx->lock, flags);
}

static void ucx_out_complete(struct urb *urb)
{
	struct ucx_urb *u = urb->context;
	struct ucx *ucx = u->ucx;
	bool elapsed = false;
	unsigned long flags;
	int i, err;

	if (ucx_urb_killed(urb->status))
		return;
	spin_lock_irqsave(&ucx->lock, flags);
	if (!ucx->streaming)
		goto unlock;
	ucx_lag(ucx, u, true);
	for (i = 0; i < urb->number_of_packets; i++) {
		if (urb->iso_frame_desc[i].status) {
			ucx->stats.out_errors++;
			ucx->stats.out_status = urb->iso_frame_desc[i].status;
		}
	}
	ucx_fill_out(ucx, u, &elapsed);
	err = ucx_submit(ucx, u, true, false);
	if (err)
		ucx_fail(ucx, err);
	if (elapsed)
		ucx_elapsed(&ucx->playback);
unlock:
	spin_unlock_irqrestore(&ucx->lock, flags);
}

static int ucx_endpoint_size(struct usb_host_interface *alt, u8 address)
{
	unsigned int i;

	for (i = 0; alt && i < alt->desc.bNumEndpoints; i++) {
		const struct usb_endpoint_descriptor *d = &alt->endpoint[i].desc;

		if (d->bEndpointAddress == address)
			return usb_endpoint_maxp(d) * usb_endpoint_maxp_mult(d);
	}
	return -ENODEV;
}

static void ucx_free_urb(struct ucx *ucx, struct ucx_urb *u)
{
	if (!u->urb)
		return;
	if (u->urb->transfer_buffer)
		usb_free_coherent(ucx->dev, u->bytes, u->urb->transfer_buffer, u->urb->transfer_dma);
	usb_free_urb(u->urb);
	u->urb = NULL;
}

static int ucx_alloc_urbs(struct ucx *ucx, bool out)
{
	struct ucx_urb *set = out ? ucx->out : ucx->in;
	unsigned int size = out ? ucx->out_size : ucx->in_size, i, j;

	for (i = 0; i < ucx->nurbs; i++) {
		struct ucx_urb *u = &set[i];
		struct urb *urb = usb_alloc_urb(ucx->packets, GFP_KERNEL);

		if (!urb)
			return -ENOMEM;
		u->ucx = ucx;
		u->urb = urb;
		u->bytes = ucx->packets * size;
		urb->transfer_buffer = usb_alloc_coherent(ucx->dev, u->bytes, GFP_KERNEL,
							  &urb->transfer_dma);
		if (!urb->transfer_buffer)
			return -ENOMEM;
		urb->dev = ucx->dev;
		urb->pipe = out ? usb_sndisocpipe(ucx->dev, UCX_EP_OUT) :
				  usb_rcvisocpipe(ucx->dev, UCX_EP_IN);
		/* and no URB_ISO_ASAP: see the top */
		urb->transfer_flags = URB_NO_TRANSFER_DMA_MAP;
		urb->transfer_buffer_length = u->bytes;
		urb->number_of_packets = ucx->packets;
		urb->interval = 1;
		urb->complete = out ? ucx_out_complete : ucx_in_complete;
		urb->context = u;
		for (j = 0; j < ucx->packets; j++) {
			urb->iso_frame_desc[j].offset = j * size;
			urb->iso_frame_desc[j].length = out ? 0 : size;
		}
	}
	return 0;
}

/*
 * Under mutex. alt0 puts interface 0 back at alt 0, except while suspending: that creates
 * endpoint devices, which the USB core doesn't do for an interface prepared for sleep. The next
 * start selects its alt setting anyway.
 */
static void ucx_engine_stop(struct ucx *ucx, bool alt0)
{
	unsigned int i;
	u32 mask;

	spin_lock_irq(&ucx->lock);
	ucx->streaming = false;
	spin_unlock_irq(&ucx->lock);
	for (i = 0; i < UCX_MAX_URBS; i++) {
		if (ucx->in[i].urb)
			usb_kill_urb(ucx->in[i].urb);
		if (ucx->out[i].urb)
			usb_kill_urb(ucx->out[i].urb);
	}
	if (ucx->running && !ucx->disconnected) {
		mask = ((1U << ucx->channels) - 1) << 2;
		ucx_write(ucx, UCX_REQ_MUTE, mask >> 16, mask & 0xffff);
		if (alt0)
			usb_set_interface(ucx->dev, 0, 0);
	}
	for (i = 0; i < UCX_MAX_URBS; i++) {
		ucx_free_urb(ucx, &ucx->in[i]);
		ucx_free_urb(ucx, &ucx->out[i]);
	}
	ucx->running = false;
}

/*
 * Under mutex: streams both ways at `rate`, as ucx-stream's Engine::run starts: alt setting,
 * base rate, stream start requests, then the URBs. OUT packets stay empty until the clock is
 * steady.
 */
static int ucx_engine_start(struct ucx *ucx, unsigned int rate)
{
	unsigned int speed = ucx_speed(rate), alt = speed == 4 ? 3 : speed, i;
	struct usb_host_interface *as;
	int err = 0, in, out;
	bool unused = false;
	u32 mask;

	if (ucx->running && (ucx->broken || ucx->rate_running != rate))
		ucx_engine_stop(ucx, true);
	if (ucx->running)
		return 0;
	if (ucx->disconnected)
		return -ENODEV;

	as = usb_altnum_to_altsetting(ucx->intf, alt);
	in = ucx_endpoint_size(as, UCX_EP_IN);
	out = ucx_endpoint_size(as, UCX_EP_OUT);
	if (in < 0 || out < 0)
		return -ENODEV;
	ucx->rate_running = rate;
	ucx->channels = ucx_channels(rate);
	ucx->in_size = in;
	ucx->out_size = out;
	ucx->max_frames = out / ((ucx->channels + 1) * 4);
	ucx->packets = clamp(READ_ONCE(packets), 1U, (unsigned int)UCX_MAX_PACKETS);
	ucx->nurbs = clamp(READ_ONCE(urbs), 2U, (unsigned int)UCX_MAX_URBS);

	err = usb_set_interface(ucx->dev, 0, alt);
	if (err)
		return err;
	mask = ((1U << ucx->channels) - 1) << 2;
	err = ucx_write(ucx, UCX_REQ_CONTROL, ucx_base_rate(rate) << 4, 0x0030);
	if (!err)
		err = ucx_write(ucx, UCX_REQ_CONTROL, 0x8000, 0x8000);
	if (!err)
		err = ucx_write(ucx, UCX_REQ_STREAM, 0, 0);
	if (!err)
		err = ucx_write(ucx, UCX_REQ_UNMUTE, mask >> 16, mask & 0xffff);
	if (!err)
		err = ucx_alloc_urbs(ucx, false);
	if (!err)
		err = ucx_alloc_urbs(ucx, true);
	ucx->running = true;	/* for ucx_engine_stop's clean-up from here on */
	if (err)
		goto fail;

	spin_lock_irq(&ucx->lock);
	ucx->streaming = true;
	ucx->broken = false;
	ucx->in_next = 0;
	ucx->have_position = false;
	ucx->steady = 0;
	ucx->clock_ok = false;
	ucx->out_locked = false;
	ucx->capture_started = false;
	ucx->stats.in_start = -1;
	ucx->stats.out_start = -1;
	ucx->stats.have_lag = false;
	ucx->stats.starts++;
	for (i = 0; i < ucx->nurbs && !err; i++) {
		err = ucx_submit(ucx, &ucx->in[i], false, !i);
		if (!i && !err)
			ucx->stats.in_start = ucx->in[0].urb->start_frame;
	}
	for (i = 0; i < ucx->nurbs && !err; i++) {
		ucx_fill_out(ucx, &ucx->out[i], &unused);
		err = ucx_submit(ucx, &ucx->out[i], true, !i);
		if (!i && !err)
			ucx->stats.out_start = ucx->out[0].urb->start_frame;
	}
	spin_unlock_irq(&ucx->lock);
	if (err)
		goto fail;
	dev_info(&ucx->intf->dev, "%u Hz, %u channels: IN from microframe %d, OUT from %d\n",
		 rate, ucx->channels, ucx->stats.in_start, ucx->stats.out_start);
	return 0;

fail:
	dev_err(&ucx->intf->dev, "starting at %u Hz: %d\n", rate, err);
	spin_lock_irq(&ucx->lock);
	ucx_xrun(ucx);	/* the other direction ran on the engine this replaced */
	spin_unlock_irq(&ucx->lock);
	ucx_engine_stop(ucx, true);
	return err;
}

static const struct snd_pcm_hardware ucx_pcm_hw = {
	.info = SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED |
		SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_BATCH,
	.formats = SNDRV_PCM_FMTBIT_FLOAT_LE,
	.rates = SNDRV_PCM_RATE_KNOT,
	.rate_min = 32000,
	.rate_max = 192000,
	.channels_min = 12,
	.channels_max = 18,
	.buffer_bytes_max = 1024 * 1024,
	.period_bytes_min = 1024,
	.period_bytes_max = 512 * 1024,
	.periods_min = 2,
	.periods_max = 1024,
};

static struct ucx_pcm *ucx_pcm_of(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);

	return ss->stream == SNDRV_PCM_STREAM_PLAYBACK ? &ucx->playback : &ucx->capture;
}

/* Channels follow from the rate: 18 at 1x, 14 at 2x, 12 at 4x. */
static int ucx_rule_channels(struct snd_pcm_hw_params *params, struct snd_pcm_hw_rule *rule)
{
	struct snd_interval *r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
	struct snd_interval c = {
		.min = ucx_channels(r->max), .max = ucx_channels(r->min), .integer = 1,
	};

	return snd_interval_refine(hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS), &c);
}

static int ucx_rule_rate(struct snd_pcm_hw_params *params, struct snd_pcm_hw_rule *rule)
{
	struct snd_interval *c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
	struct snd_interval r = { .min = UINT_MAX, .max = 0, .integer = 1 };
	unsigned int i;

	for (i = 0; i < ARRAY_SIZE(ucx_rates); i++) {
		unsigned int n = ucx_channels(ucx_rates[i]);

		if (n >= c->min && n <= c->max) {
			r.min = min(r.min, ucx_rates[i]);
			r.max = max(r.max, ucx_rates[i]);
		}
	}
	if (r.min > r.max)
		return -EINVAL;
	return snd_interval_refine(hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE), &r);
}

/* While the other direction has its params, this one runs at its rate. */
static int ucx_rule_other_rate(struct snd_pcm_hw_params *params, struct snd_pcm_hw_rule *rule)
{
	struct snd_pcm_substream *ss = rule->private;
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);
	struct snd_interval r = { .integer = 1 };

	if (READ_ONCE(ucx->users) <= (READ_ONCE(p->has_params) ? 1 : 0))
		return 0;
	r.min = READ_ONCE(ucx->rate);
	r.max = r.min;
	return snd_interval_refine(hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE), &r);
}

static int ucx_pcm_open(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct snd_pcm_runtime *rt = ss->runtime;
	struct ucx_pcm *p = ucx_pcm_of(ss);
	unsigned int urb_us;
	int err;

	/* (the constraint helpers return > 0 when they narrow something) */
	rt->hw = ucx_pcm_hw;
	err = snd_pcm_hw_constraint_list(rt, 0, SNDRV_PCM_HW_PARAM_RATE, &ucx_rate_list);
	if (err >= 0)
		err = snd_pcm_hw_rule_add(rt, 0, SNDRV_PCM_HW_PARAM_CHANNELS, ucx_rule_channels,
					  NULL, SNDRV_PCM_HW_PARAM_RATE, -1);
	if (err >= 0)
		err = snd_pcm_hw_rule_add(rt, 0, SNDRV_PCM_HW_PARAM_RATE, ucx_rule_rate, NULL,
					  SNDRV_PCM_HW_PARAM_CHANNELS, -1);
	if (err >= 0)
		err = snd_pcm_hw_rule_add(rt, 0, SNDRV_PCM_HW_PARAM_RATE, ucx_rule_other_rate, ss,
					  SNDRV_PCM_HW_PARAM_RATE, -1);
	if (err >= 0)
		err = snd_pcm_hw_constraint_integer(rt, SNDRV_PCM_HW_PARAM_PERIODS);
	/*
	 * hw_ptr moves a URB at a time, so a period is at least one: then a buffer (two periods or
	 * more) never wraps within one completion.
	 */
	mutex_lock(&ucx->mutex);
	urb_us = (ucx->running ? ucx->packets :
		  clamp(READ_ONCE(packets), 1U, (unsigned int)UCX_MAX_PACKETS)) * UCX_UFRAME_NS / 1000;
	mutex_unlock(&ucx->mutex);
	if (err >= 0)
		err = snd_pcm_hw_constraint_minmax(rt, SNDRV_PCM_HW_PARAM_PERIOD_TIME, urb_us,
						   UINT_MAX);
	if (err < 0)
		return err;

	spin_lock_irq(&ucx->lock);
	p->substream = ss;
	p->active = false;
	spin_unlock_irq(&ucx->lock);
	return 0;
}

static int ucx_pcm_close(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);

	/* completions call snd_pcm_period_elapsed under lock, so none runs after this */
	spin_lock_irq(&ucx->lock);
	p->substream = NULL;
	p->active = false;
	spin_unlock_irq(&ucx->lock);
	return 0;
}

static int ucx_pcm_hw_params(struct snd_pcm_substream *ss, struct snd_pcm_hw_params *hw)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);
	unsigned int rate = params_rate(hw);
	int err = 0;

	mutex_lock(&ucx->mutex);
	if (ucx->users > (p->has_params ? 1 : 0) && rate != ucx->rate) {
		err = -EBUSY;	/* the other direction runs at another rate */
	} else {
		if (!p->has_params)
			ucx->users++;
		p->has_params = true;
		ucx->rate = rate;
	}
	mutex_unlock(&ucx->mutex);
	return err;
}

static int ucx_pcm_hw_free(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);

	mutex_lock(&ucx->mutex);
	if (p->has_params) {
		p->has_params = false;
		if (!--ucx->users)
			ucx_engine_stop(ucx, true);
	}
	mutex_unlock(&ucx->mutex);
	return 0;
}

static int ucx_pcm_prepare(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);
	int err;

	mutex_lock(&ucx->mutex);
	err = ucx_engine_start(ucx, ss->runtime->rate);
	mutex_unlock(&ucx->mutex);
	if (err)
		return err;
	spin_lock_irq(&ucx->lock);
	p->hw_ptr = 0;
	p->period_pos = 0;
	p->delay = 0;
	if (p == &ucx->capture)
		ucx->capture_started = false;
	spin_unlock_irq(&ucx->lock);
	return 0;
}

/* Under the stream lock, so it doesn't take ucx->lock (completions take them the other way). */
static int ucx_pcm_trigger(struct snd_pcm_substream *ss, int cmd)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);
	struct ucx_pcm *p = ucx_pcm_of(ss);

	switch (cmd) {
	case SNDRV_PCM_TRIGGER_START:
		/* an engine that lost a URB or failed to start: prepare again, which restarts it */
		if (READ_ONCE(ucx->broken) || !READ_ONCE(ucx->streaming))
			return -EPIPE;
		WRITE_ONCE(p->active, true);
		return 0;
	case SNDRV_PCM_TRIGGER_STOP:
	case SNDRV_PCM_TRIGGER_SUSPEND:
		WRITE_ONCE(p->active, false);
		return 0;
	default:
		return -EINVAL;
	}
}

/*
 * After a stop, before ALSA frees or reallocates the buffer (hw_free, hw_params, prepare,
 * suspend, disconnect): a completion that saw the stream active has finished with the buffer
 * once it lets go of the lock, and later ones see it inactive.
 */
static int ucx_pcm_sync_stop(struct snd_pcm_substream *ss)
{
	struct ucx *ucx = snd_pcm_substream_chip(ss);

	spin_lock_irq(&ucx->lock);
	spin_unlock_irq(&ucx->lock);
	return 0;
}

static snd_pcm_uframes_t ucx_pcm_pointer(struct snd_pcm_substream *ss)
{
	struct ucx_pcm *p = ucx_pcm_of(ss);

	if (ss->stream == SNDRV_PCM_STREAM_PLAYBACK)
		ss->runtime->delay = READ_ONCE(p->delay);
	return READ_ONCE(p->hw_ptr);
}

static const struct snd_pcm_ops ucx_pcm_ops = {
	.open = ucx_pcm_open,
	.close = ucx_pcm_close,
	.hw_params = ucx_pcm_hw_params,
	.hw_free = ucx_pcm_hw_free,
	.prepare = ucx_pcm_prepare,
	.trigger = ucx_pcm_trigger,
	.sync_stop = ucx_pcm_sync_stop,
	.pointer = ucx_pcm_pointer,
};

/* Prints a time in ns as microframes with two decimals. */
static void ucx_print_uframes(struct snd_info_buffer *buf, const char *what, s64 ns)
{
	s64 hundredths = div_s64(ns * 100, UCX_UFRAME_NS);
	s32 rest;
	s64 whole = div_s64_rem(abs(hundredths), 100, &rest);

	snd_iprintf(buf, "%s%s%lld.%02d microframes\n", what, hundredths < 0 ? "-" : "", whole,
		    rest);
}

/* /proc/asound/cardN/ucx: the engine's state and counters. */
static void ucx_proc_read(struct snd_info_entry *entry, struct snd_info_buffer *buf)
{
	struct ucx *ucx = entry->private_data;
	struct ucx_stats s;
	bool streaming, clock_ok, out_locked;
	s64 position, lead_next;
	unsigned int rate, channels, n, nurbs;

	mutex_lock(&ucx->mutex);
	rate = ucx->running ? ucx->rate_running : 0;
	channels = ucx->channels;
	n = ucx->packets;
	nurbs = ucx->nurbs;
	spin_lock_irq(&ucx->lock);
	s = ucx->stats;
	streaming = ucx->streaming;
	clock_ok = ucx->clock_ok;
	out_locked = ucx->out_locked;
	position = ucx->pos_after;
	lead_next = ucx->lead_next;
	spin_unlock_irq(&ucx->lock);
	mutex_unlock(&ucx->mutex);

	snd_iprintf(buf, "firmware %08x\n", ucx->firmware);
	if (!rate) {
		snd_iprintf(buf, "stopped (%llu starts)\n", s.starts);
		return;
	}
	snd_iprintf(buf, "%u Hz, %u channels, %u URBs of %u microframes per direction, lead %u\n",
		    rate, channels, nurbs, n, READ_ONCE(lead_frames));
	snd_iprintf(buf, "streaming %d, clock %s, OUT %s; %llu starts\n", streaming,
		    clock_ok ? "steady" : "settling", out_locked ? "locked" : "silent", s.starts);
	snd_iprintf(buf, "first URBs: IN start_frame %d, OUT %d\n", s.in_start, s.out_start);
	snd_iprintf(buf, "IN: %llu packets, %llu errors (last %d), %llu position jumps, %llu gaps\n",
		    s.in_packets, s.in_errors, s.in_status, s.jumps, s.in_gaps);
	snd_iprintf(buf, "OUT: %llu errors (last %d), %llu gaps, %llu resyncs\n",
		    s.out_errors, s.out_status, s.out_gaps, s.resyncs);
	snd_iprintf(buf, "position %lld, next OUT frame %lld\n", position, lead_next);
	snd_iprintf(buf, "capture: %llu frames filled, %llu resyncs\n", s.filled, s.capture_resyncs);
	if (s.have_lag && s.in_lag != S64_MAX && s.out_lag != S64_MAX) {
		/*
		 * Each stream's earliest completion against the end of its microframes, as
		 * ucx-stream lines the streams up: OUT minus IN is near 0 if start_frame tells
		 * the microframes right.
		 */
		ucx_print_uframes(buf, "OUT against IN by completion times: ", s.out_lag - s.in_lag);
	}
}

static struct usb_driver ucx_driver;

/*
 * Interface 2's MIDI ports on the card, before it's registered. Without them (midi=0, a program
 * holding the interface through usbfs, no MIDIStreaming interface) the card has audio only.
 */
static int ucx_midi_create(struct ucx *ucx)
{
	struct usb_interface *intf = usb_ifnum_to_if(ucx->dev, UCX_IF_MIDI);
	struct usb_interface_descriptor *d;
	int err;

	if (!midi || !intf)
		return 0;
	d = &intf->altsetting[0].desc;
	if (d->bInterfaceClass != USB_CLASS_AUDIO ||
	    d->bInterfaceSubClass != USB_SUBCLASS_MIDISTREAMING)
		return 0;
	err = usb_driver_claim_interface(&ucx_driver, intf, ucx);
	if (err) {
		dev_warn(&ucx->intf->dev, "MIDI interface busy (%d), no MIDI ports\n", err);
		return 0;
	}
	ucx->midi_intf = intf;
	return __snd_usbmidi_create(ucx->card, intf, &ucx->midi_list, NULL, 0, NULL);
}

/*
 * Stops MIDI's URBs and lets go of interface 2 unless that's `going` (being unbound already).
 * Its disconnect, if called for this, finds no driver data and returns.
 */
static void ucx_midi_release(struct ucx *ucx, struct usb_interface *going)
{
	struct usb_interface *intf = ucx->midi_intf;
	struct list_head *p;

	list_for_each(p, &ucx->midi_list)
		snd_usbmidi_disconnect(p);
	if (!intf)
		return;
	ucx->midi_intf = NULL;
	usb_set_intfdata(intf, NULL);
	if (intf != going)
		usb_driver_release_interface(&ucx_driver, intf);
}

static int ucx_probe(struct usb_interface *intf, const struct usb_device_id *usb_id)
{
	struct usb_device *dev = interface_to_usbdev(intf);
	u32 firmware = 0, status = 0;
	struct snd_card *card;
	struct ucx *ucx;
	char path[32];
	int i, err;

	if (dev->speed != USB_SPEED_HIGH)
		return -ENODEV;

	mutex_lock(&ucx_devices_mutex);
	for (i = 0; i < SNDRV_CARDS; i++)
		if (enable[i] && !ucx_used[i])
			break;
	if (i >= SNDRV_CARDS) {
		err = -ENODEV;
		goto unlock;
	}
	err = snd_card_new(&intf->dev, index[i], id[i], THIS_MODULE, sizeof(*ucx), &card);
	if (err)
		goto unlock;
	ucx = card->private_data;
	ucx->dev = dev;
	ucx->intf = intf;
	ucx->card = card;
	ucx->card_index = i;
	INIT_LIST_HEAD(&ucx->midi_list);
	mutex_init(&ucx->mutex);
	spin_lock_init(&ucx->lock);

	if (!ucx_read(ucx, UCX_REQ_FIRMWARE, &firmware))
		ucx->firmware = swab32(firmware);
	ucx_read(ucx, UCX_REQ_STATUS, &status);
	usb_make_path(dev, path, sizeof(path));
	strscpy(card->driver, "FirefaceUCX", sizeof(card->driver));
	strscpy(card->shortname, "Fireface UCX", sizeof(card->shortname));
	snprintf(card->longname, sizeof(card->longname), "RME Fireface UCX %s at %s",
		 dev->serial ? dev->serial : "", path);

	err = usb_set_interface(dev, 0, 0);
	if (err)
		goto free;
	err = snd_pcm_new(card, "Fireface UCX", 0, 1, 1, &ucx->pcm);
	if (err)
		goto free;
	ucx->pcm->private_data = ucx;
	strscpy(ucx->pcm->name, "Fireface UCX", sizeof(ucx->pcm->name));
	snd_pcm_set_ops(ucx->pcm, SNDRV_PCM_STREAM_PLAYBACK, &ucx_pcm_ops);
	snd_pcm_set_ops(ucx->pcm, SNDRV_PCM_STREAM_CAPTURE, &ucx_pcm_ops);
	err = snd_pcm_set_managed_buffer_all(ucx->pcm, SNDRV_DMA_TYPE_VMALLOC, NULL, 0, 0);
	if (err)
		goto free;
	err = snd_card_ro_proc_new(card, "ucx", ucx, ucx_proc_read);
	if (err)
		goto free;
	err = ucx_midi_create(ucx);
	if (err)
		goto free;
	err = snd_card_register(card);
	if (err)
		goto free;

	ucx_used[i] = true;
	usb_set_intfdata(intf, ucx);
	mutex_unlock(&ucx_devices_mutex);
	dev_info(&intf->dev, "card %d: firmware %08x, status %08x, %s\n", card->number,
		 ucx->firmware, status, ucx->midi_intf ? "MIDI ports" : "no MIDI");
	return 0;

free:
	ucx_midi_release(ucx, NULL);
	snd_card_free(card);
unlock:
	mutex_unlock(&ucx_devices_mutex);
	return err;
}

/*
 * Either interface going takes the whole card, as in snd-ua101: the first one's call does it all
 * and lets go of the other, whose call then finds no driver data. (On unplug and rmmod that's
 * interface 0's; interface 2 goes first only when it alone is unbound, by sysfs or a program
 * detaching it for usbfs, and then the audio interface is let go too.)
 */
static void ucx_disconnect(struct usb_interface *intf)
{
	struct ucx *ucx = usb_get_intfdata(intf);

	if (!ucx)
		return;
	usb_set_intfdata(ucx->intf, NULL);
	snd_card_disconnect(ucx->card);
	mutex_lock(&ucx->mutex);
	ucx->disconnected = true;
	ucx_engine_stop(ucx, true);
	mutex_unlock(&ucx->mutex);
	ucx_midi_release(ucx, intf);
	mutex_lock(&ucx_devices_mutex);
	ucx_used[ucx->card_index] = false;
	mutex_unlock(&ucx_devices_mutex);
	if (intf != ucx->intf)
		usb_driver_release_interface(&ucx_driver, ucx->intf);
	snd_card_free_when_closed(ucx->card);
}

/*
 * Called for each interface. Audio streams stop; applications prepare again after resume, which
 * restarts them. MIDI input stops and starts again.
 */
static int ucx_suspend(struct usb_interface *intf, pm_message_t message)
{
	struct ucx *ucx = usb_get_intfdata(intf);
	struct list_head *p;

	if (!ucx)
		return 0;
	if (intf == ucx->midi_intf) {
		list_for_each(p, &ucx->midi_list)
			snd_usbmidi_suspend(p);
		return 0;
	}
	snd_power_change_state(ucx->card, SNDRV_CTL_POWER_D3hot);
	snd_pcm_suspend_all(ucx->pcm);
	mutex_lock(&ucx->mutex);
	ucx_engine_stop(ucx, false);
	mutex_unlock(&ucx->mutex);
	return 0;
}

static int ucx_resume(struct usb_interface *intf)
{
	struct ucx *ucx = usb_get_intfdata(intf);
	struct list_head *p;

	if (!ucx)
		return 0;
	if (intf == ucx->midi_intf) {
		list_for_each(p, &ucx->midi_list)
			snd_usbmidi_resume(p);
		return 0;
	}
	snd_power_change_state(ucx->card, SNDRV_CTL_POWER_D0);
	return 0;
}

static const struct usb_device_id ucx_ids[] = {
	{ USB_DEVICE_INTERFACE_NUMBER(UCX_VENDOR, UCX_PRODUCT, 0) },
	{ }
};
MODULE_DEVICE_TABLE(usb, ucx_ids);

static struct usb_driver ucx_driver = {
	.name = "snd-fireface-ucx",
	.probe = ucx_probe,
	.disconnect = ucx_disconnect,
	.suspend = ucx_suspend,
	.resume = ucx_resume,
	.reset_resume = ucx_resume,
	.id_table = ucx_ids,
};
module_usb_driver(ucx_driver);

MODULE_DESCRIPTION("RME Fireface UCX audio in vendor mode");
MODULE_LICENSE("GPL");
