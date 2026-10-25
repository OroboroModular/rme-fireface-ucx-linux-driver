"""Capture USB traffic of the Fireface UCX only, with USBPcap.

    python tools/capture.py --start-service       # once per sitting: the only UAC prompt
    python tools/capture.py re/captures/06-matrix.pcap            # stop with Ctrl+C
    python tools/capture.py re/captures/00-test.pcap --seconds 5
    python tools/capture.py re/captures/01-init-a.pcap --seconds 90 --follow   # power-cycle the UCX meanwhile
    python tools/capture.py --stop-service

USBPcap needs admin rights. The capture service is an elevated process that takes capture
requests and writes the captures into C:\\ProgramData\\ucx-capture, a folder only admins can write
to; the client then copies them to the path you asked for. A request can only name the output
file and set --seconds, --follow and --snaplen; the service finds the USB hub and address itself
and runs USBPcap only from Program Files. It quits after --hours (default 6) or --stop-service.
Without the service running, each capture asks for elevation itself.

Captures only the UCX: the same hub also carries keyboard and mouse, so never capture the whole
hub. Normally USBPcap filters by the UCX's address in the kernel.

--follow is for power-cycling the UCX during a capture (that's how to record its init). The UCX
comes back at a new, unpredictable address, which the kernel-side filter can't follow. So this
mode captures the whole hub into memory and filters there: it keeps packets from the UCX's
addresses (looked up every 0.5 s) and drops everything else before it reaches disk. (Restarting
the driver with pnputil also re-enumerates the UCX, but USBPcap then loses the device entirely.)

Writes <out>.log with the wall-clock start/stop times, to line the capture up with tools/mark.py.
Create <out>.stop to end a capture from another process.
"""

import argparse
import ctypes
import ctypes.wintypes as wt
import json
import os
import re
import shutil
import struct
import subprocess
import sys
import threading
import time
import uuid

VID, PID = 0x2A39, 0x3FC9
DEVICE_NAME = "RME Fireface UCX"
# elevated code must only run binaries a normal user can't swap, so not the AppData extcap copy
USBPCAPCMD = r"C:\Program Files\USBPcap\USBPcapCMD.exe"
SYSTEM32 = os.path.join(os.environ.get("SystemRoot", r"C:\Windows"), "System32")
SERVICE_DIR = os.path.join(os.environ.get("ProgramData", r"C:\ProgramData"), "ucx-capture")
REQUEST_DIR = os.path.join(os.environ.get("LOCALAPPDATA", ""), "ucx-capture", "requests")
NAME_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,80}$")
MAX_SECONDS = 3600


def find_ucx(required=True):
    """Return (interface, UCX address, addresses of every other device on that hub)."""
    ifaces = re.findall(r"\{value=(\\\\\.\\USBPcap\d+)\}", subprocess.run([USBPCAPCMD, "--extcap-interfaces"], capture_output=True, text=True).stdout)
    for iface in ifaces:
        cfg = subprocess.run([USBPCAPCMD, "--extcap-interface", iface, "--extcap-config"], capture_output=True, text=True).stdout
        m = re.search(r"\{value=(\d+)\}\{display=\[\d+\] " + re.escape(DEVICE_NAME) + r"\}", cfg)
        if m:
            others = sorted(set(re.findall(r"\{value=(\d+)\}\{display=\[", cfg)) - {m.group(1)}, key=int)
            return iface, m.group(1), others
    if not required:
        return None
    sys.exit(f"{DEVICE_NAME} not found behind any USBPcap root hub (is USBPcap running? reboot after installing)")


def read_exact(f, n):
    buf = b""
    while len(buf) < n:
        chunk = f.read(n - len(buf))
        if not chunk:
            break
        buf += chunk
    return buf


def filter_stream(src, dst_path, ucx, others, log, counts):
    """Copy a USBPcap pcap stream to dst_path, keeping only packets from UCX addresses.

    Addresses that are neither known UCX nor known other devices are buffered until they
    identify themselves, by their own device descriptor or by the caller adding them to ucx.
    """
    with open(dst_path, "wb") as out:
        hdr = read_exact(src, 24)
        if len(hdr) < 24:
            return
        out.write(hdr)
        pending = {}  # unidentified new address -> buffered records
        while True:
            rh = read_exact(src, 16)
            if len(rh) < 16:
                break
            incl = struct.unpack_from("<I", rh, 8)[0]
            data = read_exact(src, incl)
            if len(data) < incl:
                break
            if len(data) < 27:  # USBPCAP_BUFFER_PACKET_HEADER is 27 bytes
                continue
            dev = struct.unpack_from("<H", data, 19)[0]
            counts[dev] = counts.get(dev, 0) + 1
            if dev in ucx:
                if dev in pending:  # learned by the caller since we buffered these
                    out.writelines(pending.pop(dev))
                out.write(rh + data)
                continue
            if dev in others:
                continue
            buf = pending.setdefault(dev, [])
            buf.append(rh + data)
            payload = data[struct.unpack_from("<H", data, 0)[0]:]
            # control completion (info bit 0 = PDO->FDO) carrying a full device descriptor
            if data[22] == 2 and data[16] & 1 and len(payload) >= 18 and payload[0] == 18 and payload[1] == 1:
                if struct.unpack_from("<HH", payload, 8) == (VID, PID):
                    ucx.add(dev)
                    out.writelines(buf)
                    log.write(f"follow\t{time.time():.3f}\tUCX now at address {dev}\n")
                    log.flush()
                else:
                    others.add(dev)
                del pending[dev]
            elif len(buf) > 200000:  # never identified; not the UCX we're following
                others.add(dev)
                del pending[dev]


def run_capture(out, seconds, follow, snaplen, stop_requested, log):
    """Capture the UCX into out until seconds pass (0 = no limit) or stop_requested(). Runs elevated."""
    found = find_ucx(required=False)
    if not found:
        log.write(f"error\t{time.time():.3f}\t{DEVICE_NAME} not found on any USBPcap root hub\n")
        return False
    iface, addr, others = found
    if follow:
        # whole hub to our stdout, filtered in memory by filter_stream
        cmd = [USBPCAPCMD, "-d", iface, "-A", "--capture-from-new-devices", "--inject-descriptors", "-o", "-"]
    else:
        cmd = [USBPCAPCMD, "-d", iface, "--devices", addr, "--inject-descriptors", "-o", out]
    if snaplen:
        cmd += ["-s", str(snaplen)]
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE if follow else log, stderr=log,
                            cwd=os.path.dirname(out), creationflags=subprocess.CREATE_NO_WINDOW)
    ucx, counts = {int(addr)}, {}
    if follow:
        reader = threading.Thread(target=filter_stream, args=(proc.stdout, out, ucx, {int(x) for x in others}, log, counts), daemon=True)
        reader.start()
    t0 = time.time()
    log.write(f"start\t{t0:.3f}\t{DEVICE_NAME} is device {addr} on {iface}\t{' '.join(cmd)}\n")
    log.flush()
    next_lookup = 0
    while proc.poll() is None:
        now = time.time()
        if follow and now >= next_lookup:
            # a replugged UCX comes back at a new address and its descriptor is read before
            # USBPcap hooks it, so ask the hub where it is
            next_lookup = now + 0.5
            moved = find_ucx(required=False)
            if moved and int(moved[1]) not in ucx:
                ucx.add(int(moved[1]))
                log.write(f"follow\t{time.time():.3f}\tUCX now at address {moved[1]}\n")
                log.flush()
        if stop_requested() or (seconds and now - t0 >= seconds):
            break
        time.sleep(0.1)
    early = proc.poll()
    if early is None:
        proc.terminate()
        proc.wait()
    if follow:
        reader.join(5)
        kept = sum(n for d, n in counts.items() if d in ucx)
        log.write(f"packets per address (kept: {sorted(ucx)}, {kept} packets)\t{dict(sorted(counts.items()))}\n")
    log.write(f"stop\t{time.time():.3f}\t{'exited by itself, code ' + str(early) if early is not None else 'terminated'}\n")
    log.flush()
    return True


# --- elevation ---------------------------------------------------------------------------------

class SHELLEXECUTEINFOW(ctypes.Structure):
    _fields_ = [
        ("cbSize", wt.DWORD), ("fMask", ctypes.c_ulong), ("hwnd", wt.HWND), ("lpVerb", wt.LPCWSTR),
        ("lpFile", wt.LPCWSTR), ("lpParameters", wt.LPCWSTR), ("lpDirectory", wt.LPCWSTR), ("nShow", ctypes.c_int),
        ("hInstApp", wt.HINSTANCE), ("lpIDList", ctypes.c_void_p), ("lpClass", wt.LPCWSTR), ("hkeyClass", wt.HKEY),
        ("dwHotKey", wt.DWORD), ("hIconOrMonitor", wt.HANDLE), ("hProcess", wt.HANDLE),
    ]


def run_elevated(args):
    """Relaunch this script elevated with args, return its process handle."""
    sei = SHELLEXECUTEINFOW(cbSize=ctypes.sizeof(SHELLEXECUTEINFOW), fMask=0x40, lpVerb="runas", lpFile=sys.executable,
                            lpParameters=subprocess.list2cmdline(["-I", os.path.abspath(__file__)] + args), nShow=0)
    if not ctypes.windll.shell32.ShellExecuteExW(ctypes.byref(sei)):
        sys.exit("elevation was declined or failed (UAC prompts only show on the PC's own screen)")
    return sei.hProcess


def child(a):
    """Elevated side of a one-off capture (no service running)."""
    stop_file = a.out + ".stop"
    with open(a.out + ".log", "w", encoding="utf-8") as log:
        run_capture(a.out, a.seconds, a.follow, a.snaplen, lambda: os.path.exists(stop_file), log)
    if os.path.exists(stop_file):
        os.remove(stop_file)


# --- capture service ---------------------------------------------------------------------------

def secure_service_dir():
    """Make SERVICE_DIR a fresh folder that only admins can write to (the user can read)."""
    if os.path.lexists(SERVICE_DIR):
        attrs = ctypes.windll.kernel32.GetFileAttributesW(SERVICE_DIR)
        if attrs == 0xFFFFFFFF or attrs & 0x400 or not attrs & 0x10:  # reparse point or not a directory
            raise SystemExit(f"{SERVICE_DIR} is not a plain directory; refusing to use it")
    else:
        os.mkdir(SERVICE_DIR)
    sid = subprocess.run([os.path.join(SYSTEM32, "whoami.exe"), "/user", "/fo", "csv", "/nh"],
                         capture_output=True, text=True, check=True).stdout.strip().split(",")[-1].strip('"')
    icacls = os.path.join(SYSTEM32, "icacls.exe")
    subprocess.run([icacls, SERVICE_DIR, "/setowner", "*S-1-5-32-544", "/T", "/C", "/Q"], capture_output=True)
    subprocess.run([icacls, SERVICE_DIR, "/inheritance:r", "/grant:r", "*S-1-5-32-544:(OI)(CI)F", "*S-1-5-18:(OI)(CI)F",
                    f"*{sid}:(OI)(CI)RX", "/T", "/C", "/Q"], capture_output=True, check=True)
    # anything planted before the ACL applied (links, junctions) goes; links are removed, not followed
    for name in os.listdir(SERVICE_DIR):
        p = os.path.join(SERVICE_DIR, name)
        if os.path.islink(p) or os.path.isjunction(p):
            os.rmdir(p) if os.path.isdir(p) else os.unlink(p)
        elif os.path.isdir(p):
            shutil.rmtree(p)
        else:
            os.remove(p)


def service(a):
    """Elevated capture service: run validated requests from REQUEST_DIR, one at a time."""
    secure_service_dir()
    os.makedirs(a.requests, exist_ok=True)
    quit_file = os.path.join(a.requests, "quit")
    if os.path.exists(quit_file):
        os.remove(quit_file)
    slog = open(os.path.join(SERVICE_DIR, "service.log"), "w", encoding="utf-8")
    deadline = time.time() + a.hours * 3600
    slog.write(f"started\t{time.time():.3f}\tuntil {time.strftime('%H:%M', time.localtime(deadline))}\trequests from {a.requests}\n")
    slog.flush()
    alive = threading.Event()
    alive.set()

    def heartbeat():
        while alive.is_set():
            with open(os.path.join(SERVICE_DIR, "heartbeat"), "w") as f:
                f.write(f"{time.time():.3f}\t{deadline:.0f}\n")
            time.sleep(1)

    threading.Thread(target=heartbeat, daemon=True).start()
    try:
        while time.time() < deadline and not os.path.exists(quit_file):
            for entry in sorted(os.listdir(a.requests)):
                if entry.endswith(".json"):
                    handle_request(a.requests, entry, quit_file, slog)
            time.sleep(0.5)
    finally:
        alive.clear()
        time.sleep(1.1)
        os.remove(os.path.join(SERVICE_DIR, "heartbeat"))
        slog.write(f"stopped\t{time.time():.3f}\n")
        slog.close()
        if os.path.exists(quit_file):
            os.remove(quit_file)


def handle_request(req_dir, entry, quit_file, slog):
    rid = entry[:-5]
    path = os.path.join(req_dir, entry)
    try:
        with open(path, encoding="utf-8") as f:
            spec = json.load(f)
    except (OSError, ValueError):
        spec = None
    os.remove(path)
    if not re.fullmatch(r"[0-9a-f]{32}", rid):
        return
    error = None
    if not isinstance(spec, dict) or not NAME_RE.match(str(spec.get("name", ""))):
        error = "bad request"
    else:
        try:
            seconds, follow, snaplen = float(spec.get("seconds", 0)), bool(spec.get("follow")), int(spec.get("snaplen", 0))
        except (TypeError, ValueError):
            error = "bad request"
        else:
            if not 0 <= seconds <= MAX_SECONDS or not 0 <= snaplen <= 65535:
                error = "bad request"
    ok = False
    if not error:
        out = os.path.join(SERVICE_DIR, spec["name"] + ".pcap")
        for p in (out, out + ".log"):
            if os.path.lexists(p):
                os.remove(p)
        stop_file = os.path.join(req_dir, rid + ".stop")
        slog.write(f"capture\t{time.time():.3f}\t{spec['name']} seconds={seconds} follow={follow} snaplen={snaplen}\n")
        slog.flush()
        with open(out + ".log", "w", encoding="utf-8") as log:
            ok = run_capture(out, seconds or MAX_SECONDS, follow, snaplen,
                             lambda: os.path.exists(stop_file) or os.path.exists(quit_file), log)
        if os.path.exists(stop_file):
            os.remove(stop_file)
    with open(os.path.join(SERVICE_DIR, rid + ".done"), "w", encoding="utf-8") as f:
        json.dump({"ok": ok, "error": error}, f)


def service_alive():
    try:
        return time.time() - os.path.getmtime(os.path.join(SERVICE_DIR, "heartbeat")) < 5
    except OSError:
        return False


def start_service(hours):
    if service_alive():
        print(f"capture service already running ({SERVICE_DIR})")
        return
    os.makedirs(REQUEST_DIR, exist_ok=True)
    h = run_elevated(["--service", "--requests", REQUEST_DIR, "--hours", str(hours)])
    ctypes.windll.kernel32.CloseHandle(h)
    print("starting capture service (accept the UAC prompt on the PC)")
    for _ in range(600):
        if service_alive():
            print(f"capture service running for {hours} h; captures need no more prompts until then")
            return
        time.sleep(0.2)
    sys.exit("capture service did not come up")


def stop_service():
    if not service_alive():
        print("capture service not running")
        return
    open(os.path.join(REQUEST_DIR, "quit"), "w").close()
    for _ in range(100):
        if not service_alive():
            print("capture service stopped")
            return
        time.sleep(0.2)
    sys.exit("capture service did not stop")


def capture_via_service(a):
    name = os.path.splitext(os.path.basename(a.out))[0]
    if not NAME_RE.match(name):
        sys.exit(f"output name {name!r} must be letters, digits, '.', '_' or '-'")
    rid = uuid.uuid4().hex
    tmp = os.path.join(REQUEST_DIR, rid + ".tmp")
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump({"name": name, "seconds": a.seconds, "follow": a.follow, "snaplen": a.snaplen}, f)
    os.replace(tmp, os.path.join(REQUEST_DIR, rid + ".json"))
    src = os.path.join(SERVICE_DIR, name + ".pcap")
    done = os.path.join(SERVICE_DIR, rid + ".done")
    print(f"capturing via the capture service to {a.out}" + ("" if a.seconds else "; press Ctrl+C to stop"))
    last_log = None

    def mirror_log():
        nonlocal last_log
        try:
            text = open(src + ".log", encoding="utf-8").read()
        except OSError:
            return
        if text != last_log:
            last_log = text
            with open(a.out + ".log", "w", encoding="utf-8") as f:
                f.write(text)

    while not os.path.exists(done):
        try:
            mirror_log()
            if os.path.exists(a.out + ".stop"):
                open(os.path.join(REQUEST_DIR, rid + ".stop"), "w").close()
                os.remove(a.out + ".stop")
            if not service_alive():
                sys.exit("capture service went away")
            time.sleep(0.2)
        except KeyboardInterrupt:
            open(os.path.join(REQUEST_DIR, rid + ".stop"), "w").close()
    result = json.load(open(done, encoding="utf-8"))
    mirror_log()
    if result.get("error"):
        sys.exit(f"capture service refused the request: {result['error']}")
    if os.path.exists(src):
        shutil.copyfile(src, a.out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out", nargs="?", help="output .pcap path")
    ap.add_argument("--seconds", type=float, default=0, help="stop after this long (default: until Ctrl+C)")
    ap.add_argument("--snaplen", type=int, default=0, help="truncate packets to this many bytes")
    ap.add_argument("--follow", action="store_true", help="keep capturing the UCX when it replugs at a new address")
    ap.add_argument("--start-service", action="store_true", help="start the elevated capture service (one UAC prompt)")
    ap.add_argument("--stop-service", action="store_true", help="stop the capture service")
    ap.add_argument("--hours", type=float, default=6, help="capture service lifetime (default 6)")
    ap.add_argument("--child", action="store_true", help=argparse.SUPPRESS)
    ap.add_argument("--service", action="store_true", help=argparse.SUPPRESS)
    ap.add_argument("--requests", help=argparse.SUPPRESS)
    a = ap.parse_args()

    if a.service:
        return service(a)
    if a.start_service:
        return start_service(a.hours)
    if a.stop_service:
        return stop_service()
    if not a.out:
        ap.error("output path required")
    a.out = os.path.abspath(a.out)
    if a.child:
        return child(a)

    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    for stale in (a.out, a.out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    if service_alive():
        capture_via_service(a)
    else:
        print(f"no capture service running; capturing to {a.out} with its own UAC prompt")
        args = [a.out, "--child", "--seconds", str(a.seconds), "--snaplen", str(a.snaplen)]
        if a.follow:
            args.append("--follow")
        h = run_elevated(args)
        print("capturing (accept the UAC prompt)" + ("" if a.seconds else "; press Ctrl+C to stop"))
        try:
            while ctypes.windll.kernel32.WaitForSingleObject(h, 200) == 0x102:  # WAIT_TIMEOUT
                pass
        except KeyboardInterrupt:
            open(a.out + ".stop", "w").close()
            ctypes.windll.kernel32.WaitForSingleObject(h, 10000)
        ctypes.windll.kernel32.CloseHandle(h)
    if os.path.exists(a.out + ".log"):
        print(open(a.out + ".log", encoding="utf-8").read().rstrip())
    size = os.path.getsize(a.out) if os.path.exists(a.out) else 0
    print(f"{a.out}: {size} bytes")
    if not size:
        sys.exit(1)


if __name__ == "__main__":
    main()
