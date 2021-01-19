'''
File: nativemethods.py
Project: ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 19/01/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import ctypes
from ctypes import c_void_p, c_int, POINTER, c_ulong, Structure, c_double, c_bool


class Vector3(Structure):
    _fields_ = [("x", c_double), ("y", c_double), ("z", c_double)]

    def __init__(self, position):
        super().__init__()
        self.x = position[0]
        self.y = position[1]
        self.z = position[2]


class WaveSource(Structure):
    _fields_ = [("x", c_double), ("y", c_double), ("z", c_double), ("amp", c_double), ("phase", c_double)]

    def __init__(self, position, amp, phase):
        super().__init__()
        self.pos = position
        self.amp = amp
        self.phase = phase

    @property
    def pos(self):
        pass

    @pos.setter
    def pos(self, position):
        self.x = position[0]
        self.y = position[1]
        self.z = position[2]

    @pos.getter
    def pos(self):
        return (self.x, self.y, self.z)


def init_dll(dll_location):
    global GHR_DLL  # pylint: disable=global-variable-undefined
    GHR_DLL = ctypes.CDLL(dll_location)

    __init_calculator()
    __init_builder()
    __init_buffer()
    __init_optimizer()


def __init_calculator():
    GHR_DLL.GHR_CreateCpuCalculator.argtypes = [POINTER(c_void_p)]
    GHR_DLL.GHR_CreateCpuCalculator.restypes = [None]

    GHR_DLL.GHR_FreeCalculator.argtypes = [c_void_p]
    GHR_DLL.GHR_FreeCalculator.restypes = [None]

    GHR_DLL.GHR_AddWaveSource.argtypes = [c_void_p, WaveSource]
    GHR_DLL.GHR_AddWaveSource.restypes = [None]

    GHR_DLL.GHR_InitWaveSources.argtypes = [c_void_p, c_ulong]
    GHR_DLL.GHR_InitWaveSources.restypes = [None]

    GHR_DLL.GHR_WaveSources.argtypes = [c_void_p, POINTER(c_void_p)]
    GHR_DLL.GHR_WaveSources.restypes = [c_ulong]


def __init_builder():
    GHR_DLL.GHR_CreateBufferBuilder.argtypes = [POINTER(c_void_p)]
    GHR_DLL.GHR_CreateBufferBuilder.restypes = []

    GHR_DLL.GHR_FreeBufferBuilder.argtypes = [c_void_p]
    GHR_DLL.GHR_FreeBufferBuilder.restypes = [None]

    GHR_DLL.GHR_BufferBuilder_At.argtypes = [POINTER(c_void_p), c_int, c_double]
    GHR_DLL.GHR_BufferBuilder_At.restypes = [None]

    GHR_DLL.GHR_BufferBuilder_Range.argtypes = [POINTER(c_void_p), c_int, c_double, c_double]
    GHR_DLL.GHR_BufferBuilder_Range.restypes = [None]

    GHR_DLL.GHR_BufferBuilder_Resolution.argtypes = [POINTER(c_void_p), c_double]
    GHR_DLL.GHR_BufferBuilder_Resolution.restypes = [None]

    GHR_DLL.GHR_BufferBuilder_Generate.argtypes = [c_void_p, c_int, POINTER(c_void_p)]
    GHR_DLL.GHR_BufferBuilder_Generate.restypes = [None]


def __init_buffer():
    GHR_DLL.GHR_FreeBuffer.argtypes = [c_void_p]
    GHR_DLL.GHR_FreeBuffer.restypes = [None]

    GHR_DLL.GHR_GetScalarBufferArray.argtypes = [c_void_p, POINTER(c_void_p), c_int]
    GHR_DLL.GHR_GetScalarBufferArray.restypes = [c_ulong]

    GHR_DLL.GHR_GetScalarMax.argtypes = [c_void_p, c_int]
    GHR_DLL.GHR_GetScalarMax.restypes = [c_double]

    GHR_DLL.GHR_GetBounds.argtypes = [c_void_p, c_int, POINTER(c_ulong), POINTER(c_ulong), POINTER(c_ulong)]
    GHR_DLL.GHR_GetBounds.restypes = [None]

    GHR_DLL.GHR_GetDimension.argtypes = [c_void_p, c_int, POINTER(c_int), POINTER(c_int), POINTER(c_int)]
    GHR_DLL.GHR_GetDimension.restypes = [None]

    GHR_DLL.GHR_Calculate.argtypes = [c_void_p, c_void_p, c_int]
    GHR_DLL.GHR_Calculate.restypes = [None]


def __init_optimizer():
    GHR_DLL.GHR_GreedyBruteForce.argtypes = [c_void_p, POINTER(c_double), POINTER(c_double), c_ulong, c_ulong, c_ulong, c_bool, c_bool]
    GHR_DLL.GHR_GreedyBruteForce.restypes = [None]

    GHR_DLL.GHR_Horn.argtypes = [c_void_p, POINTER(c_double), POINTER(c_double), c_ulong, c_ulong, c_double, c_double]
    GHR_DLL.GHR_Horn.restypes = [None]

    GHR_DLL.GHR_Long.argtypes = [c_void_p, POINTER(c_double), POINTER(c_double), c_ulong, c_double]
    GHR_DLL.GHR_Long.restypes = [None]

    GHR_DLL.GHR_LM.argtypes = [c_void_p, POINTER(c_double), POINTER(c_double), c_ulong, c_double, c_double, c_double, c_ulong]
    GHR_DLL.GHR_LM.restypes = [None]

    GHR_DLL.GHR_GSPAT.argtypes = [c_void_p, POINTER(c_double), POINTER(c_double), c_ulong, c_ulong]
    GHR_DLL.GHR_GSPAT.restypes = [None]
