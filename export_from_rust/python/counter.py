from ctypes import CDLL, c_void_p, c_int

class Counter:

    def __init__(self, lib):
        self.lib = CDLL(lib)
        self.lib.alloc.restype = c_void_p
        self.lib.dealloc.argtypes = [c_void_p]
        self.lib.clear.argtypes = [c_void_p]
        self.lib.inc.argtypes = [c_void_p]
        self.lib.value.argtypes = [c_void_p]
        self.lib.value.restype = c_int
        self.handle = self.lib.alloc()

    def __del__(self):
        self.lib.dealloc(self.handle)

    def clear(self):
        self.lib.clear(self.handle)

    def inc(self):
        self.lib.inc(self.handle)

    def value(self):
        return self.lib.value(self.handle)

