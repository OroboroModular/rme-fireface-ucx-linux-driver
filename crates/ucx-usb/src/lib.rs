//! The UCX through Linux usbfs: ioctls on its device node, as `linux/usbfs.py` does it. Shared by
//! `ucx-stream` (iso audio on interface 0) and the mixer (interface 1, vendor requests).
//!
//! Iso URBs go in without `USBDEVFS_URB_ISO_ASAP`, so the xHCI driver starts each stream on a USB
//! frame boundary. usbfs doesn't report the frame a URB went out in, so each one is stamped with
//! the time it was reaped instead (`re/NOTES.md`, "OUT timing").
//!
//! The kernel copies results into a URB's memory only while it is being reaped, at the addresses
//! given at submit time. So a URB's memory must stay put until it has been reaped, even after a
//! discard: [`IsoUrb`] keeps it in boxes, and its owner reaps everything before dropping it.

use std::ffi::{c_int, c_ulong, c_void};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use ucx_protocol::control::VendorWrite;

pub const VID: u16 = 0x2a39;
pub const PID: u16 = 0x3fc9;

/// usbfs takes at most this many packets per iso URB.
pub const MAX_PACKETS: usize = 128;

#[repr(C)]
struct CtrlTransfer {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
    timeout_ms: u32,
    data: *mut c_void,
}

#[repr(C)]
struct BulkTransfer {
    endpoint: u32,
    length: u32,
    timeout_ms: u32,
    data: *mut c_void,
}

#[repr(C)]
struct SetInterface {
    interface: u32,
    alt: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct IsoPacketDesc {
    length: u32,
    actual_length: u32,
    status: i32,
}

/// `struct usbdevfs_urb`, without its trailing `iso_frame_desc[]`.
#[repr(C)]
struct Urb {
    kind: u8,
    endpoint: u8,
    status: i32,
    flags: u32,
    buffer: *mut c_void,
    buffer_length: i32,
    actual_length: i32,
    start_frame: i32,
    number_of_packets: i32,
    error_count: i32,
    signr: u32,
    usercontext: *mut c_void,
}

/// A URB followed by its packet descriptors, as the kernel reads it.
#[repr(C)]
struct UrbMem {
    urb: Urb,
    desc: [IsoPacketDesc; MAX_PACKETS],
}

const URB_TYPE_ISO: u8 = 0;

const fn ioc(dir: u32, nr: u32, size: usize) -> c_ulong {
    // asm-generic/ioctl.h: _IOC_WRITE = 1, _IOC_READ = 2
    (dir << 30 | (size as u32) << 16 | (b'U' as u32) << 8 | nr) as c_ulong
}

const CONTROL: c_ulong = ioc(3, 0, size_of::<CtrlTransfer>());
const BULK: c_ulong = ioc(3, 2, size_of::<BulkTransfer>());
const SETINTERFACE: c_ulong = ioc(2, 4, size_of::<SetInterface>());
const SUBMITURB: c_ulong = ioc(2, 10, size_of::<Urb>());
const DISCARDURB: c_ulong = ioc(0, 11, 0);
const REAPURBNDELAY: c_ulong = ioc(1, 13, size_of::<*mut c_void>());
const CLAIMINTERFACE: c_ulong = ioc(2, 15, size_of::<u32>());
const RELEASEINTERFACE: c_ulong = ioc(2, 16, size_of::<u32>());

// the values linux/usbfs.py checked against the kernel headers
const _: () = assert!(CONTROL == 0xc018_5500 && SETINTERFACE == 0x8008_5504 && SUBMITURB == 0x8038_550a);
const _: () = assert!(REAPURBNDELAY == 0x4008_550d && CLAIMINTERFACE == 0x8004_550f);
const _: () = assert!(BULK == 0xc018_5502);

/// Nanoseconds on CLOCK_MONOTONIC.
pub fn now_ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

/// One iso URB: `packets` packets of up to `size` bytes each, with its buffer.
pub struct IsoUrb {
    mem: Box<UrbMem>,
    buf: Box<[u8]>,
    pub endpoint: u8,
    pub packets: usize,
    pub size: usize,
    /// The caller's index of the first packet.
    pub base: u64,
    /// [`now_ns`] right after the URB was reaped.
    pub reaped_at: u64,
    pub pending: bool,
}

impl IsoUrb {
    /// `tag` comes back from [`Device::reap`].
    pub fn new(endpoint: u8, packets: usize, size: usize, tag: usize) -> Self {
        assert!(packets <= MAX_PACKETS);
        let mut buf = vec![0u8; packets * size].into_boxed_slice();
        let urb = Urb {
            kind: URB_TYPE_ISO,
            endpoint,
            status: 0,
            flags: 0,
            buffer: buf.as_mut_ptr().cast(),
            buffer_length: 0,
            actual_length: 0,
            start_frame: 0,
            number_of_packets: packets as i32,
            error_count: 0,
            signr: 0,
            usercontext: tag as *mut c_void,
        };
        let mem = Box::new(UrbMem { urb, desc: [IsoPacketDesc::default(); MAX_PACKETS] });
        IsoUrb { mem, buf, endpoint, packets, size, base: 0, reaped_at: 0, pending: false }
    }

    /// OUT: the packets back to back. `write(i, room)` writes packet i into `room` (`size`
    /// bytes) and returns its length.
    pub fn fill(&mut self, mut write: impl FnMut(usize, &mut [u8]) -> usize) {
        let mut offset = 0;
        for i in 0..self.packets {
            let n = write(i, &mut self.buf[offset..offset + self.size]);
            assert!(n <= self.size);
            self.mem.desc[i].length = n as u32;
            offset += n;
        }
        self.mem.urb.buffer_length = offset as i32;
    }

    /// IN: room for a full packet in every slot.
    pub fn expect(&mut self) {
        for d in &mut self.mem.desc[..self.packets] {
            d.length = self.size as u32;
        }
        self.mem.urb.buffer_length = (self.packets * self.size) as i32;
    }

    /// After a reap: packet i's status (0 or a negative errno).
    pub fn status(&self, i: usize) -> i32 {
        self.mem.desc[i].status
    }

    /// IN, after a reap: packet i's data. Each packet sits at i × `size`.
    pub fn received(&self, i: usize) -> &[u8] {
        let n = self.mem.desc[i].actual_length as usize;
        &self.buf[i * self.size..i * self.size + n.min(self.size)]
    }
}

pub struct Device {
    file: File,
    pub node: PathBuf,
    claimed: Vec<u32>,
}

fn sysfs_hex(dir: &Path, name: &str) -> Option<u16> {
    u16::from_str_radix(fs::read_to_string(dir.join(name)).ok()?.trim(), 16).ok()
}

fn sysfs_dec(dir: &Path, name: &str) -> Option<u32> {
    fs::read_to_string(dir.join(name)).ok()?.trim().parse().ok()
}

/// The first 2a39:3fc9 on the bus, as its usbfs node.
pub fn find_node() -> Option<PathBuf> {
    for entry in fs::read_dir("/sys/bus/usb/devices").ok()?.flatten() {
        let dir = entry.path();
        if sysfs_hex(&dir, "idVendor") == Some(VID) && sysfs_hex(&dir, "idProduct") == Some(PID) {
            let (bus, dev) = (sysfs_dec(&dir, "busnum")?, sysfs_dec(&dir, "devnum")?);
            return Some(PathBuf::from(format!("/dev/bus/usb/{bus:03}/{dev:03}")));
        }
    }
    None
}

impl Device {
    pub fn open() -> io::Result<Device> {
        let node = find_node()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("no {VID:04x}:{PID:04x} on the bus")))?;
        let file = OpenOptions::new().read(true).write(true).open(&node).map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                io::Error::new(e.kind(), format!("no write access to {}: install linux/70-fireface-ucx.rules", node.display()))
            } else {
                e
            }
        })?;
        Ok(Device { file, node, claimed: Vec::new() })
    }

    fn ioctl(&self, request: c_ulong, arg: *mut c_void) -> io::Result<c_int> {
        let rc = unsafe { libc::ioctl(self.file.as_raw_fd(), request, arg) };
        if rc < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(rc)
        }
    }

    fn control(&self, request_type: u8, request: u8, value: u16, index: u16, data: &mut [u8]) -> io::Result<usize> {
        let mut ct = CtrlTransfer {
            request_type,
            request,
            value,
            index,
            length: data.len() as u16,
            timeout_ms: 1000,
            data: data.as_mut_ptr().cast(),
        };
        let n = self.ioctl(CONTROL, (&raw mut ct).cast()).map_err(|e| {
            io::Error::new(e.kind(), format!("request {request} {value:04x}/{index:04x}: {e}"))
        })?;
        Ok(n as usize)
    }

    /// A vendor IN request with a 4-byte answer, as a little-endian u32.
    pub fn read(&self, request: u8) -> io::Result<u32> {
        let mut b = [0u8; 4];
        let n = self.control(0xc0, request, 0, 0, &mut b)?;
        if n != 4 {
            return Err(io::Error::other(format!("request {request}: {n} bytes")));
        }
        Ok(u32::from_le_bytes(b))
    }

    pub fn write(&self, w: VendorWrite) -> io::Result<()> {
        self.control(0x40, w.request, w.value, w.index, &mut []).map(drop)
    }

    /// A bulk transfer, waiting up to `timeout_ms`: an IN endpoint reads into `data`, an OUT one
    /// sends it. Returns the length moved; a timeout is `ErrorKind::TimedOut`.
    pub fn bulk(&self, endpoint: u8, data: &mut [u8], timeout_ms: u32) -> io::Result<usize> {
        let mut bt =
            BulkTransfer { endpoint: endpoint as u32, length: data.len() as u32, timeout_ms, data: data.as_mut_ptr().cast() };
        let n = self
            .ioctl(BULK, (&raw mut bt).cast())
            .map_err(|e| io::Error::new(e.kind(), format!("bulk on {endpoint:#04x}: {e}")))?;
        Ok(n as usize)
    }

    pub fn claim(&mut self, interface: u32) -> io::Result<()> {
        let mut i = interface;
        self.ioctl(CLAIMINTERFACE, (&raw mut i).cast())
            .map_err(|e| io::Error::new(e.kind(), format!("claim interface {interface}: {e}")))?;
        self.claimed.push(interface);
        Ok(())
    }

    pub fn set_alt(&self, interface: u32, alt: u32) -> io::Result<()> {
        let mut si = SetInterface { interface, alt };
        self.ioctl(SETINTERFACE, (&raw mut si).cast())
            .map_err(|e| io::Error::new(e.kind(), format!("interface {interface} alt {alt}: {e}")))
            .map(drop)
    }

    pub fn submit(&self, urb: &mut IsoUrb) -> io::Result<()> {
        assert!(!urb.pending);
        self.ioctl(SUBMITURB, (&raw mut urb.mem.urb).cast())
            .map_err(|e| io::Error::new(e.kind(), format!("submit on {:#04x}: {e}", urb.endpoint)))?;
        urb.pending = true;
        Ok(())
    }

    /// Cancels a pending URB; it still has to be reaped. Fails only if it completed meanwhile.
    pub fn discard(&self, urb: &mut IsoUrb) {
        if urb.pending {
            let _ = self.ioctl(DISCARDURB, (&raw mut urb.mem.urb).cast());
        }
    }

    /// The tag of a completed URB, if any, without waiting. The caller looks the URB up by its
    /// tag and marks it reaped.
    pub fn reap(&self) -> io::Result<Option<usize>> {
        let mut p: *mut UrbMem = std::ptr::null_mut();
        match self.ioctl(REAPURBNDELAY, (&raw mut p).cast()) {
            Ok(_) => Ok(Some(unsafe { (*p).urb.usercontext } as usize)),
            Err(e) if e.raw_os_error() == Some(libc::EAGAIN) => Ok(None),
            Err(e) => Err(io::Error::new(e.kind(), format!("reap: {e}"))),
        }
    }

    /// Waits up to `timeout_ms` for a URB to complete. False on timeout or a signal.
    pub fn wait(&self, timeout_ms: i32) -> io::Result<bool> {
        let mut pfd = libc::pollfd { fd: self.file.as_raw_fd(), events: libc::POLLOUT, revents: 0 };
        match unsafe { libc::poll(&mut pfd, 1, timeout_ms) } {
            n if n > 0 => {
                if pfd.revents & (libc::POLLERR | libc::POLLHUP) != 0 {
                    return Err(io::Error::new(io::ErrorKind::NotConnected, "the unit went away"));
                }
                Ok(true)
            }
            0 => Ok(false),
            _ => {
                let e = io::Error::last_os_error();
                if e.kind() == io::ErrorKind::Interrupted {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        for &i in &self.claimed {
            let mut i = i;
            let _ = self.ioctl(RELEASEINTERFACE, (&raw mut i).cast());
        }
    }
}
