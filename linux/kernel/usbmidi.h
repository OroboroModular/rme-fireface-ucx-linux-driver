/* SPDX-License-Identifier: GPL-2.0 */
/*
 * snd-usbmidi-lib's interface, as in sound/usb/midi.h (kernel 7.0), which isn't in the kernel
 * headers packages; only what fireface_ucx.c uses. A NULL quirk means a standard MIDIStreaming
 * interface, its endpoints and jacks read from the class descriptors.
 */
#ifndef UCX_USBMIDI_H
#define UCX_USBMIDI_H

#include <linux/list.h>

struct snd_card;
struct snd_usb_audio_quirk;
struct usb_interface;

int __snd_usbmidi_create(struct snd_card *card, struct usb_interface *iface,
			 struct list_head *midi_list, const struct snd_usb_audio_quirk *quirk,
			 unsigned int usb_id, unsigned int *num_rawmidis);
void snd_usbmidi_disconnect(struct list_head *p);
void snd_usbmidi_suspend(struct list_head *p);
void snd_usbmidi_resume(struct list_head *p);

#endif
