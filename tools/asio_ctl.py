"""Read and set the UCX's sample rate and clock source through RME's ASIO driver.

    python tools/asio_ctl.py status
    python tools/asio_ctl.py rate 96000
    python tools/asio_ctl.py clock 0          # index from "status"

A minimal ASIO host: loads the installed "ASIO Fireface USB" driver as a COM object and calls
its public IASIO methods (init, getSampleRate, canSampleRate, setSampleRate, getClockSources,
setClockSource). No audio buffers are created. Only one ASIO host can use the driver at a time,
so this fails while a DAW has it open.
"""

import ctypes
import ctypes.wintypes as wt
import sys
import time
import winreg

DRIVER_NAME = "ASIO Fireface USB"
RATES = [32000, 44100, 48000, 64000, 88200, 96000, 128000, 176400, 192000]


class GUID(ctypes.Structure):
    _fields_ = [("Data1", wt.DWORD), ("Data2", wt.WORD), ("Data3", wt.WORD), ("Data4", ctypes.c_ubyte * 8)]


class ASIOClockSource(ctypes.Structure):
    _fields_ = [("index", ctypes.c_long), ("associatedChannel", ctypes.c_long), ("associatedGroup", ctypes.c_long),
                ("isCurrentSource", ctypes.c_long), ("name", ctypes.c_char * 32)]


# IASIO vtable slots after IUnknown's three
SLOTS = {"init": 3, "getDriverName": 4, "getDriverVersion": 5, "getErrorMessage": 6, "start": 7, "stop": 8,
         "getChannels": 9, "getLatencies": 10, "getBufferSize": 11, "canSampleRate": 12, "getSampleRate": 13,
         "setSampleRate": 14, "getClockSources": 15, "setClockSource": 16, "getSamplePosition": 17,
         "getChannelInfo": 18, "createBuffers": 19, "disposeBuffers": 20, "controlPanel": 21, "future": 22,
         "outputReady": 23}
ASE_OK, ASE_SUCCESS = 0, 0x3F4847A0
ole32 = ctypes.WinDLL("ole32")


def clsid():
    with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, rf"SOFTWARE\ASIO\{DRIVER_NAME}") as k:
        s = winreg.QueryValueEx(k, "CLSID")[0]
    g = GUID()
    ole32.CLSIDFromString(ctypes.c_wchar_p(s), ctypes.byref(g))
    return g


class Asio:
    def __init__(self):
        ole32.CoInitialize(None)
        g = clsid()
        self.ptr = ctypes.c_void_p()
        # ASIO drivers use their CLSID as the interface id
        hr = ole32.CoCreateInstance(ctypes.byref(g), None, 1, ctypes.byref(g), ctypes.byref(self.ptr))
        if hr != 0 or not self.ptr:
            raise OSError(f"CoCreateInstance failed: 0x{hr & 0xffffffff:08x}")
        self.vtbl = ctypes.cast(self.ptr, ctypes.POINTER(ctypes.POINTER(ctypes.c_void_p)))[0]
        hwnd = ctypes.windll.kernel32.GetConsoleWindow() or ctypes.windll.user32.GetDesktopWindow()
        if not self.call("init", ctypes.c_long, [ctypes.c_void_p], ctypes.c_void_p(hwnd)):
            buf = ctypes.create_string_buffer(124)
            self.call("getErrorMessage", None, [ctypes.c_char_p], buf)
            raise OSError(f"ASIO init failed: {buf.value.decode(errors='replace')} (is another ASIO host running?)")

    def call(self, name, restype, argtypes, *args):
        fn = ctypes.WINFUNCTYPE(restype, ctypes.c_void_p, *argtypes)(self.vtbl[SLOTS[name]])
        return fn(self.ptr, *args)

    def close(self):
        release = ctypes.WINFUNCTYPE(ctypes.c_ulong, ctypes.c_void_p)(self.vtbl[2])
        release(self.ptr)

    def rate(self):
        r = ctypes.c_double()
        self.call("getSampleRate", ctypes.c_long, [ctypes.POINTER(ctypes.c_double)], ctypes.byref(r))
        return r.value

    def can_rate(self, rate):
        return self.call("canSampleRate", ctypes.c_long, [ctypes.c_double], rate) == ASE_OK

    def set_rate(self, rate):
        return self.call("setSampleRate", ctypes.c_long, [ctypes.c_double], float(rate))

    def clocks(self):
        arr = (ASIOClockSource * 16)()
        n = ctypes.c_long(16)
        self.call("getClockSources", ctypes.c_long, [ctypes.POINTER(ASIOClockSource), ctypes.POINTER(ctypes.c_long)], arr, ctypes.byref(n))
        return [(c.index, c.name.decode(errors="replace"), bool(c.isCurrentSource)) for c in arr[:n.value]]

    def set_clock(self, index):
        return self.call("setClockSource", ctypes.c_long, [ctypes.c_long], index)


def main():
    if len(sys.argv) < 2 or sys.argv[1] not in ("status", "rate", "clock"):
        sys.exit(__doc__)
    a = Asio()
    try:
        if sys.argv[1] == "rate":
            err = a.set_rate(int(sys.argv[2]))
            time.sleep(1.0)
            print(f"setSampleRate({sys.argv[2]}) -> {err}; now {a.rate():.0f} Hz")
        elif sys.argv[1] == "clock":
            err = a.set_clock(int(sys.argv[2]))
            time.sleep(1.0)
            print(f"setClockSource({sys.argv[2]}) -> {err}")
        print(f"sample rate: {a.rate():.0f} Hz")
        print("supported:", ", ".join(str(r) for r in RATES if a.can_rate(r)))
        for index, name, current in a.clocks():
            print(f"clock source {index}: {name}{'  (current)' if current else ''}")
    finally:
        a.close()


if __name__ == "__main__":
    main()
