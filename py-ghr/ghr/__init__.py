'''
File: __init__.py
Project: ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 26/06/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import os.path

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

LIB_PATH = os.path.join(os.path.dirname(__file__), 'bin', 'ghrcapi.dll')
init_dll(LIB_PATH)
