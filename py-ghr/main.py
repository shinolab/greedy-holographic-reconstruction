'''
File: main.py
Project: py-ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 26/06/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import math
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

from ghr import CpuCalculator, BufferBuilder, plot_helper, FieldType, Optimizer  # NOQA


if __name__ == '__main__':
    NUM_TRANS_X = 18
    NUM_TRANS_Y = 14
    TRANS_SIZE = 10.18
    WAVE_LENGTH = 8.5

    # Observe properties, units are mm
    X_RANGE = (TRANS_SIZE * 8.5 - 50.0, TRANS_SIZE * 8.5 + 50.0)
    Y_RANGE = (TRANS_SIZE * 6.5 - 50.0, TRANS_SIZE * 6.5 + 50.0)
    RESOLUTION = 1.0

    # Initialize calculator
    calculator = CpuCalculator()
    calculator.set_wave_num(2.0 * math.pi / WAVE_LENGTH)

    # Initialize sound sources
    calculator.init_wave_sources(NUM_TRANS_X * NUM_TRANS_Y)

    focal_pos = np.array([TRANS_SIZE * 8.5, TRANS_SIZE * 6.5, 150.])

    # initialize position, direction, amplitude and phase of each sound source
    wave_sources = calculator.wave_sources()
    for y in range(NUM_TRANS_Y):
        for x in range(NUM_TRANS_X):
            pos = np.array([TRANS_SIZE * x, TRANS_SIZE * y, 0.])

            i = x + y * NUM_TRANS_X
            wave_sources[i].pos = pos
            wave_sources[i].amp = 0.0
            wave_sources[i].phase = 0.0
    calculator.update_amp_phase()
    calculator.update_source_geometry()

    target_pos = [
        focal_pos + np.array([20., 0., 0.]),
        focal_pos - np.array([20., 0., 0.]),
        focal_pos + np.array([0., 20., 0.]),
        focal_pos - np.array([0., 20., 0.]),
    ]
    amps = np.ones(len(target_pos))

    optimizer = Optimizer.greedy_full_search(calculator, target_pos, 1 << 4)

    # show phases
    dpi = 72
    fig = plt.figure(figsize=(6, 6), dpi=dpi)
    axes = fig.add_subplot(111, aspect='equal')

    scat = plot_helper.plot_phase_2d(fig, axes, wave_sources, TRANS_SIZE)
    plot_helper.add_colorbar(fig, axes, scat)
    plt.savefig('phase_gfs.png')
    plt.show()

    # generate buffer
    buffer = BufferBuilder.new()\
        .x_range(X_RANGE)\
        .y_range(Y_RANGE)\
        .z_at(150.0)\
        .resolution(RESOLUTION)\
        .generate(FieldType.Pressure)

    buffer.calculate(calculator)

    # plot
    bounds = buffer.bounds()
    array = buffer.get_array().reshape(bounds[0], bounds[1])
    DPI = 72
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')
    heat_map = plot_helper.plot_acoustic_field_2d(axes, array, X_RANGE, Y_RANGE, RESOLUTION, ticks_step=10.0)
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%')
    fig.colorbar(heat_map, cax=cax)
    plt.savefig('xy_gfs.png')
    plt.show()

    # ######### HORN #####################
    optimizer = Optimizer.horn(calculator, target_pos, amps, WAVE_LENGTH)

    # show phases
    dpi = 72
    fig = plt.figure(figsize=(6, 6), dpi=dpi)
    axes = fig.add_subplot(111, aspect='equal')

    scat = plot_helper.plot_phase_2d(fig, axes, wave_sources, TRANS_SIZE)
    plot_helper.add_colorbar(fig, axes, scat)
    plt.savefig('phase_horn.png')
    plt.show()

    # generate buffer
    buffer = BufferBuilder.new()\
        .x_range(X_RANGE)\
        .y_range(Y_RANGE)\
        .z_at(150.0)\
        .resolution(RESOLUTION)\
        .generate(FieldType.Pressure)

    buffer.calculate(calculator)

    # plot
    bounds = buffer.bounds()
    array = buffer.get_array().reshape(bounds[0], bounds[1])
    DPI = 72
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')
    heat_map = plot_helper.plot_acoustic_field_2d(axes, array, X_RANGE, Y_RANGE, RESOLUTION, ticks_step=10.0)
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%')
    fig.colorbar(heat_map, cax=cax)
    plt.savefig('xy_horn.png')
    plt.show()
