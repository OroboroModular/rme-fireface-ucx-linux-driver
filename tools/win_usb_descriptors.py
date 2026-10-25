"""Dump USB descriptors of an attached device on Windows, without detaching its driver.

Asks the parent hub for the descriptors (the same IOCTLs USBView uses), so the RME
driver stays bound and audio keeps running.

    python tools/win_usb_descriptors.py [--vid 2a39] [--pid 3fc9] [--out re/descriptors]
"""

import argparse
import ctypes
import ctypes.wintypes as wt
import os
import struct
import sys

setupapi = ctypes.WinDLL("setupapi", use_last_error=True)
kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)


class GUID(ctypes.Structure):
    _fields_ = [("Data1", wt.DWORD), ("Data2", wt.WORD), ("Data3", wt.WORD), ("Data4", ctypes.c_ubyte * 8)]


class SP_DEVICE_INTERFACE_DATA(ctypes.Structure):
    _fields_ = [("cbSize", wt.DWORD), ("InterfaceClassGuid", GUID), ("Flags", wt.DWORD), ("Reserved", ctypes.c_void_p)]


GUID_DEVINTERFACE_USB_HUB = GUID(0xF18A0E88, 0xC30C, 0x11D0, (ctypes.c_ubyte * 8)(0x88, 0x15, 0x00, 0xA0, 0xC9, 0x06, 0xBE, 0xD8))
DIGCF_PRESENT, DIGCF_DEVICEINTERFACE = 0x02, 0x10
GENERIC_WRITE, FILE_SHARE_WRITE, OPEN_EXISTING = 0x40000000, 0x2, 3
INVALID_HANDLE_VALUE = ctypes.c_void_p(-1).value

IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION = 0x220410
IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX = 0x220448

setupapi.SetupDiGetClassDevsW.restype = ctypes.c_void_p
setupapi.SetupDiGetClassDevsW.argtypes = [ctypes.POINTER(GUID), wt.LPCWSTR, wt.HWND, wt.DWORD]
setupapi.SetupDiEnumDeviceInterfaces.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.POINTER(GUID), wt.DWORD, ctypes.POINTER(SP_DEVICE_INTERFACE_DATA)]
setupapi.SetupDiGetDeviceInterfaceDetailW.argtypes = [ctypes.c_void_p, ctypes.POINTER(SP_DEVICE_INTERFACE_DATA), ctypes.c_void_p, wt.DWORD, ctypes.POINTER(wt.DWORD), ctypes.c_void_p]
setupapi.SetupDiDestroyDeviceInfoList.argtypes = [ctypes.c_void_p]
kernel32.CreateFileW.restype = ctypes.c_void_p
kernel32.CreateFileW.argtypes = [wt.LPCWSTR, wt.DWORD, wt.DWORD, ctypes.c_void_p, wt.DWORD, wt.DWORD, ctypes.c_void_p]
kernel32.DeviceIoControl.argtypes = [ctypes.c_void_p, wt.DWORD, ctypes.c_void_p, wt.DWORD, ctypes.c_void_p, wt.DWORD, ctypes.POINTER(wt.DWORD), ctypes.c_void_p]
kernel32.CloseHandle.argtypes = [ctypes.c_void_p]


def hub_paths():
    hdev = setupapi.SetupDiGetClassDevsW(ctypes.byref(GUID_DEVINTERFACE_USB_HUB), None, None, DIGCF_PRESENT | DIGCF_DEVICEINTERFACE)
    i = 0
    try:
        while True:
            ifd = SP_DEVICE_INTERFACE_DATA(cbSize=ctypes.sizeof(SP_DEVICE_INTERFACE_DATA))
            if not setupapi.SetupDiEnumDeviceInterfaces(hdev, None, ctypes.byref(GUID_DEVINTERFACE_USB_HUB), i, ctypes.byref(ifd)):
                return
            i += 1
            need = wt.DWORD()
            setupapi.SetupDiGetDeviceInterfaceDetailW(hdev, ctypes.byref(ifd), None, 0, ctypes.byref(need), None)
            buf = ctypes.create_string_buffer(need.value)
            # SP_DEVICE_INTERFACE_DETAIL_DATA_W.cbSize is 8 on x64 (DWORD + WCHAR[1], aligned)
            struct.pack_into("<I", buf, 0, 8 if ctypes.sizeof(ctypes.c_void_p) == 8 else 6)
            if setupapi.SetupDiGetDeviceInterfaceDetailW(hdev, ctypes.byref(ifd), buf, need, None, None):
                yield ctypes.wstring_at(ctypes.addressof(buf) + 4)
    finally:
        setupapi.SetupDiDestroyDeviceInfoList(hdev)


def ioctl(h, code, inbuf, outlen):
    out = ctypes.create_string_buffer(max(outlen, len(inbuf)))
    ctypes.memmove(out, inbuf, len(inbuf))
    got = wt.DWORD()
    if not kernel32.DeviceIoControl(h, code, out, len(inbuf), out, len(out), ctypes.byref(got), None):
        return None
    return out.raw[: got.value]


def connection_info(h, port):
    # USB_NODE_CONNECTION_INFORMATION_EX, packed: index(4) devdesc(18) cfg(1) speed(1) ishub(1) addr(2) npipes(4) status(4) pipes[n]*11
    raw = ioctl(h, IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX, struct.pack("<I", port) + bytes(31 + 32 * 11), 35 + 32 * 11)
    if raw is None or len(raw) < 35:
        return None
    cfg, speed, ishub, addr, npipes, status = struct.unpack_from("<BBBHII", raw, 22)
    pipes = [raw[35 + 11 * k : 35 + 11 * k + 7] for k in range(npipes) if len(raw) >= 35 + 11 * (k + 1)]
    return dict(devdesc=raw[4:22], cfg=cfg, speed=speed, addr=addr, status=status, pipes=pipes)


def get_descriptor(h, port, dtype, index, lang=0, length=4096):
    # USB_DESCRIPTOR_REQUEST: ConnectionIndex(4) + setup packet(8) + data
    req = struct.pack("<IBBHHH", port, 0x80, 6, (dtype << 8) | index, lang, length)
    raw = ioctl(h, IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION, req + bytes(length), 12 + length)
    return None if raw is None else raw[12:]


def string(h, port, idx):
    if not idx:
        return None
    d = get_descriptor(h, port, 3, idx, 0x0409, 255)
    return d[2 : d[0]].decode("utf-16-le", "replace") if d and len(d) >= 2 else None


EP_TYPES = ["control", "isochronous", "bulk", "interrupt"]
ISO_SYNC = ["none", "async", "adaptive", "sync"]
ISO_USAGE = ["data", "feedback", "implicit-fb data", "reserved"]
SPEEDS = {0: "low", 1: "full", 2: "high", 3: "super"}


def describe_ep(d):
    addr, attr, mps, interval = d[2], d[3], struct.unpack_from("<H", d, 4)[0], d[6]
    t = attr & 3
    s = f"EP 0x{addr:02x} {'IN ' if addr & 0x80 else 'OUT'} {EP_TYPES[t]}"
    if t == 1:
        s += f" sync={ISO_SYNC[(attr >> 2) & 3]} usage={ISO_USAGE[(attr >> 4) & 3]}"
    s += f" wMaxPacketSize=0x{mps:04x} ({mps & 0x7ff} x{((mps >> 11) & 3) + 1}) bInterval={interval}"
    return s


def describe_config(cfg):
    lines, i = [], 0
    while i + 2 <= len(cfg) and cfg[i] >= 2:
        ln, ty = cfg[i], cfg[i + 1]
        d = cfg[i : i + ln]
        hexs = " ".join(f"{b:02x}" for b in d)
        if ty == 2:
            lines.append(f"CONFIG wTotalLength={struct.unpack_from('<H', d, 2)[0]} bNumInterfaces={d[4]} bConfigurationValue={d[5]} bmAttributes=0x{d[7]:02x} MaxPower={d[8] * 2}mA")
        elif ty == 4:
            lines.append(f"  INTERFACE {d[2]} alt {d[3]}: bNumEndpoints={d[4]} class=0x{d[5]:02x} sub=0x{d[6]:02x} proto=0x{d[7]:02x}")
        elif ty == 5:
            lines.append(f"    {describe_ep(d)}")
        elif ty == 0x0B:
            lines.append(f"  IAD first={d[2]} count={d[3]} class=0x{d[4]:02x} sub=0x{d[5]:02x}")
        else:
            lines.append(f"    [type 0x{ty:02x} len {ln}] {hexs}")
        i += ln
    return lines


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--vid", default="2a39")
    ap.add_argument("--pid", default="3fc9")
    ap.add_argument("--out", default=None, help="directory to write raw .bin descriptors and a .txt summary")
    a = ap.parse_args()
    vid, pid = int(a.vid, 16), int(a.pid, 16)

    found = False
    for path in hub_paths():
        h = kernel32.CreateFileW(path, GENERIC_WRITE, FILE_SHARE_WRITE, None, OPEN_EXISTING, 0, None)
        if h in (None, INVALID_HANDLE_VALUE):
            continue
        try:
            for port in range(1, 33):
                ci = connection_info(h, port)
                if not ci or ci["status"] != 1:
                    continue
                dd = ci["devdesc"]
                dvid, dpid, bcd = struct.unpack_from("<HHH", dd, 8)
                if (dvid, dpid) != (vid, pid):
                    continue
                found = True
                out = [
                    f"hub {path} port {port} address {ci['addr']} speed {SPEEDS.get(ci['speed'], ci['speed'])} active config {ci['cfg']}",
                    f"DEVICE bcdUSB=0x{struct.unpack_from('<H', dd, 2)[0]:04x} class=0x{dd[4]:02x}/0x{dd[5]:02x}/0x{dd[6]:02x} bMaxPacketSize0={dd[7]} "
                    f"VID=0x{dvid:04x} PID=0x{dpid:04x} bcdDevice=0x{bcd:04x} bNumConfigurations={dd[17]}",
                    f"  manufacturer={string(h, port, dd[14])!r} product={string(h, port, dd[15])!r} serial={string(h, port, dd[16])!r}",
                    "OPEN PIPES (as configured by the Windows driver):",
                ] + [f"  {describe_ep(p)}" for p in ci["pipes"]]
                raws = {"device": dd}
                for c in range(dd[17]):
                    hdr = get_descriptor(h, port, 2, c, 0, 9)
                    if not hdr:
                        out.append(f"config index {c}: request failed")
                        continue
                    full = get_descriptor(h, port, 2, c, 0, struct.unpack_from("<H", hdr, 2)[0])
                    raws[f"config{c}"] = full
                    out.append(f"--- configuration index {c} ---")
                    out += describe_config(full)
                    # pick up interface string descriptors referenced by this config
                    j = 0
                    while j + 2 <= len(full) and full[j] >= 2:
                        if full[j + 1] == 4 and full[j + 8]:
                            out.append(f"  (interface {full[j + 2]} alt {full[j + 3]} name: {string(h, port, full[j + 8])!r})")
                        j += full[j]
                oq = get_descriptor(h, port, 6, 0, 0, 10)
                if oq:
                    out.append(f"DEVICE_QUALIFIER {' '.join(f'{b:02x}' for b in oq)}")
                text = "\n".join(out)
                print(text)
                if a.out:
                    os.makedirs(a.out, exist_ok=True)
                    stem = f"{vid:04x}_{pid:04x}"
                    with open(os.path.join(a.out, stem + ".txt"), "w", encoding="utf-8") as f:
                        f.write(text + "\n")
                    for k, v in raws.items():
                        with open(os.path.join(a.out, f"{stem}_{k}.bin"), "wb") as f:
                            f.write(v)
        finally:
            kernel32.CloseHandle(h)
    if not found:
        sys.exit(f"device {vid:04x}:{pid:04x} not found on any hub (or hubs not openable without elevation)")


if __name__ == "__main__":
    main()
