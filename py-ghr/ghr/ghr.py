'''
File: ghr.py
Project: ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 14/07/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


from enum import IntEnum

import ctypes
from ctypes import c_void_p, byref, c_ulong, c_int, POINTER, c_float, c_double, c_bool
import numpy as np

from . import nativemethods
from .nativemethods import WaveSource


class FieldType(IntEnum):
    Pressure = 1
    Power = 2


class Axis(IntEnum):
    X = 0
    Y = 1
    Z = 2


class Calculator:
    def __init__(self):
        self.handle = c_void_p()

    def __del__(self):
        nativemethods.GHR_DLL.GHR_FreeCalculator(self.handle)

    def add_wave_source(self, wave_source: WaveSource):
        nativemethods.GHR_DLL.GHR_AddWaveSource(self.handle, wave_source)

    def init_wave_sources(self, size: int):
        nativemethods.GHR_DLL.GHR_InitWaveSources(self.handle, c_ulong(size))

    def wave_sources(self):
        ptr = c_void_p()
        size = nativemethods.GHR_DLL.GHR_WaveSources(self.handle, byref(ptr))
        array = ctypes.cast(ptr, POINTER(WaveSource * size)).contents
        return array

    def set_wave_num(self, wave_num: float):
        nativemethods.GHR_DLL.GHR_SetWaveNum(self.handle, c_float(wave_num))

    def update_amp_phase(self):
        nativemethods.GHR_DLL.GHR_UpdateAmpPhase(self.handle)

    def update_source_geometry(self):
        nativemethods.GHR_DLL.GHR_UpdateSourceGeometry(self.handle)


class ScalarBuffer:
    def __init__(self):
        self.handle = c_void_p()
        self.field_type = 0

    def __del__(self):
        nativemethods.GHR_DLL.GHR_FreeBuffer(self.handle)

    def get_array(self):
        ptr = c_void_p()
        size = nativemethods.GHR_DLL.GHR_GetScalarBufferArray(self.handle, byref(ptr), self.field_type)
        ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.c_float))
        return np.ctypeslib.as_array(ptr, shape=(size,))

    def bounds(self):
        bound_x = c_ulong()
        bound_y = c_ulong()
        bound_z = c_ulong()
        nativemethods.GHR_DLL.GHR_GetBounds(self.handle, self.field_type, byref(bound_x), byref(bound_y), byref(bound_z))
        return (bound_x.value, bound_y.value, bound_z.value)

    def dimension(self):
        dim_x = c_int()
        dim_y = c_int()
        dim_z = c_int()
        nativemethods.GHR_DLL.GHR_GetDimension(self.handle, self.field_type, byref(dim_x), byref(dim_y), byref(dim_z))
        dim_x = dim_x.value
        dim_y = dim_y.value
        dim_z = dim_z.value
        if dim_z != -1:
            return (Axis(dim_x), Axis(dim_y), Axis(dim_z))
        elif dim_y != -1:
            return (Axis(dim_x), Axis(dim_y))
        elif dim_x != -1:
            return Axis(dim_x)
        else:
            return ()

    def calculate(self, calculator: Calculator):
        nativemethods.GHR_DLL.GHR_Calculate(calculator.handle, self.handle, self.field_type)


class BufferBuilder:
    def __init__(self, handle):
        self.handle = handle

    def __del__(self):
        if self.handle is not None:
            nativemethods.GHR_DLL.GHR_FreeBufferBuilder(self.handle)

    @staticmethod
    def new():
        handle = c_void_p()
        nativemethods.GHR_DLL.GHR_CreateBufferBuilder(byref(handle))
        return BufferBuilder(handle)

    def x_at(self, pos: float):
        nativemethods.GHR_DLL.GHR_BufferBuilder_At(byref(self.handle), 0, pos)
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def y_at(self, pos: float):
        nativemethods.GHR_DLL.GHR_BufferBuilder_At(byref(self.handle), 1, pos)
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def z_at(self, pos: float):
        nativemethods.GHR_DLL.GHR_BufferBuilder_At(byref(self.handle), 2, pos)
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def x_range(self, obs_range: (float, float)):
        nativemethods.GHR_DLL.GHR_BufferBuilder_Range(byref(self.handle), 0, obs_range[0], obs_range[1])
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def y_range(self, obs_range: (float, float)):
        nativemethods.GHR_DLL.GHR_BufferBuilder_Range(byref(self.handle), 1, obs_range[0], obs_range[1])
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def z_range(self, obs_range: (float, float)):
        nativemethods.GHR_DLL.GHR_BufferBuilder_Range(byref(self.handle), 2, obs_range[0], obs_range[1])
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def resolution(self, resolution: float):
        nativemethods.GHR_DLL.GHR_BufferBuilder_Resolution(byref(self.handle), resolution)
        handle = self.handle
        self.handle = None
        return BufferBuilder(handle)

    def generate(self, field_type: FieldType):
        buf = ScalarBuffer()
        nativemethods.GHR_DLL.GHR_BufferBuilder_Generate(self.handle, field_type, byref(buf.handle))
        self.handle = None
        buf.field_type = field_type
        return buf


class CpuCalculator(Calculator):
    def __init__(self):
        super().__init__()
        nativemethods.GHR_DLL.GHR_CreateCpuCalculator(byref(self.handle))


class Optimizer():
    @staticmethod
    def greedy_brute_force(calculate: Calculator, foci, amps, wave_len, include_amp, normalize):
        size = len(foci)
        amps = np.array(amps).astype(np.float64)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.GHR_DLL.GHR_GreedyBruteForce(calculate.handle, foci_array, amps, c_ulong(size),
                                                   c_double(wave_len), c_bool(include_amp), c_bool(normalize))

    @staticmethod
    def horn(calculate: Calculator, foci, amps, wave_len, include_amp, normalize):
        size = len(foci)
        amps = np.array(amps).astype(np.float64)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.GHR_DLL.GHR_Horn(calculate.handle, foci_array, amps, c_ulong(size), c_double(wave_len), c_bool(include_amp), c_bool(normalize))

    @staticmethod
    def long2014(calculate: Calculator, foci, amps, wave_len, include_amp, normalize):
        size = len(foci)
        amps = np.array(amps).astype(np.float64)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.GHR_DLL.GHR_Long(calculate.handle, foci_array, amps, c_ulong(size), c_double(wave_len), c_bool(include_amp), c_bool(normalize))

    @staticmethod
    def levenberg_marquardt(calculate: Calculator, foci, amps, wave_len, include_amp, normalize):
        size = len(foci)
        amps = np.array(amps).astype(np.float64)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.GHR_DLL.GHR_LM(calculate.handle, foci_array, amps, c_ulong(size), c_double(wave_len), c_bool(include_amp), c_bool(normalize))
