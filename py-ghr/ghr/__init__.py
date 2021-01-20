'''
File: __init__.py
Project: ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 20/01/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import os.path
import platform

from ghr import plot_helper
from .ghr import BufferBuilder, ScalarBuffer, CpuCalculator, Axis, FieldType
from .ghr import Optimizer
from .nativemethods import init_dll, WaveSource, Vector3

__all__ = [
    'BufferBuilder',
    'ScalarBuffer',
    'CpuCalculator',
    'plot_helper',
    'WaveSource',
    'Axis',
    'FieldType',
    'Optimizer',
    'Directivity',
    'Vector3']

PLATFORM = platform.system()
PREFIX = ''
EXT = ''
if PLATFORM == 'Windows':
    EXT = '.dll'
elif PLATFORM == 'Darwin':
    PREFIX = 'lib'
    EXT = '.dylib'
elif PLATFORM == 'Linux':
    PREFIX = 'lib'
    EXT = '.so'


LIB_PATH = os.path.join(os.path.dirname(__file__), 'bin', PREFIX + 'ghrcapi' + EXT)
init_dll(LIB_PATH)
