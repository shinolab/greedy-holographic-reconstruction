'''
File: main.py
Project: py-ghr
Created Date: 26/06/2020
Author: Shun Suzuki
-----
Last Modified: 22/01/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import os
import math
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

from ghr import CpuCalculator, BufferBuilder, plot_helper, FieldType, Optimizer

NUM_TRANS_X = 36
NUM_TRANS_Y = 36
TRANS_SIZE = 10
WAVE_LENGTH = 8.5
Z = 150
R = 100.0


def setup_pyplot():
    plt.rcParams['text.usetex'] = True
    plt.rcParams['axes.grid'] = False
    plt.rcParams['xtick.direction'] = 'in'
    plt.rcParams['ytick.direction'] = 'in'
    plt.rcParams['xtick.major.width'] = 1.0
    plt.rcParams['ytick.major.width'] = 1.0
    plt.rcParams['font.size'] = 14
    plt.rcParams['font.family'] = 'sans-serif'
    plt.rcParams['font.sans-serif'] = 'Arial'
    plt.rcParams["mathtext.fontset"] = 'stixsans'
    plt.rcParams['ps.useafm'] = True
    plt.rcParams['pdf.use14corefonts'] = True
    plt.rcParams['text.latex.preamble'] = r'\usepackage{sfmath}'


def plot_phase_xy(wave_sources, name, ext='pdf'):
    # Observe properties, units are mm
    X_RANGE = (TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0 - R / 2, TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0 + R / 2)
    Y_RANGE = (TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0 - R / 2, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0 + R / 2)
    RESOLUTION = 0.5

    # show phases
    DPI = 300
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')

    scat = plot_helper.plot_phase_2d(fig, axes, wave_sources, TRANS_SIZE)
    plot_helper.add_colorbar(fig, axes, scat)
    plt.savefig(name + '_phase.' + ext)

    # generate buffer
    buffer = BufferBuilder.new()\
        .x_range(X_RANGE)\
        .y_range(Y_RANGE)\
        .z_at(Z)\
        .resolution(RESOLUTION)\
        .generate(FieldType.Power)
    buffer.calculate(calculator)

    # plot
    ticks_step = 10.0
    bounds = buffer.bounds()
    array = buffer.get_array().reshape(bounds[0], bounds[1])
    print(name, ': ', array.max())
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')
    heat_map = plot_helper.plot_acoustic_field_2d(axes, array, X_RANGE, Y_RANGE, RESOLUTION, ticks_step=ticks_step)
    x_label_num = int(math.floor((X_RANGE[1] - X_RANGE[0]) / ticks_step)) + 1
    y_label_num = int(math.floor((Y_RANGE[1] - Y_RANGE[0]) / ticks_step)) + 1
    x_labels = [-(X_RANGE[1] - X_RANGE[0]) / 2 + ticks_step * i for i in range(x_label_num)]
    y_labels = [-(Y_RANGE[1] - Y_RANGE[0]) / 2 + ticks_step * i for i in range(y_label_num)]
    axes.set_xticklabels(x_labels, minor=False, fontsize=12)
    axes.set_yticklabels(y_labels, minor=False, fontsize=12)
    plt.xlabel(r'$x$\,[mm]')
    plt.ylabel(r'$y$\,[mm]')

    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%')
    fig.colorbar(heat_map, cax=cax)
    plt.tight_layout()
    plt.savefig(name + '_xy.' + ext)


def plot_target_xy(target_pos, amp, img_dir, ext='pdf'):
    # Observe properties, units are mm
    X_RANGE = (TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0 - R / 2, TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0 + R / 2)
    Y_RANGE = (TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0 - R / 2, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0 + R / 2)

    # plot
    DPI = 300
    ticks_step = 10.0
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')

    scat_x = list(map(lambda s: s[0] - TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0, target_pos))
    scat_y = list(map(lambda s: s[1] - TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0, target_pos))
    scat = axes.scatter(scat_x, scat_y, s=400, c='black', marker='.', vmin=0, vmax=amp.max())

    x_label_num = int(math.floor((X_RANGE[1] - X_RANGE[0]) / ticks_step)) + 1
    y_label_num = int(math.floor((Y_RANGE[1] - Y_RANGE[0]) / ticks_step)) + 1
    x_labels = [-(X_RANGE[1] - X_RANGE[0]) / 2 + ticks_step * i for i in range(x_label_num)]
    y_labels = [-(Y_RANGE[1] - Y_RANGE[0]) / 2 + ticks_step * i for i in range(y_label_num)]
    axes.set_xticks(x_labels, minor=False)
    axes.set_yticks(y_labels, minor=False)
    axes.set_xticklabels(x_labels, minor=False, fontsize=12)
    axes.set_yticklabels(y_labels, minor=False, fontsize=12)
    axes.set_xlim((-50, 50))
    axes.set_ylim((-50, 50))

    plt.xlabel(r'$x$\,[mm]')
    plt.ylabel(r'$y$\,[mm]')

    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%', alpha=0.2)
    cb = fig.colorbar(scat, cax=cax)
    cb.outline.set_edgecolor([0, 0, 0, 0.0])
    cb.ax.tick_params(axis='both', colors=[0, 0, 0, 0.0])
    cb.ax.yaxis.label.set_color([0, 0, 0, 0.0])
    cb.solids.set_alpha(0)
    cb.patch.set_alpha(0)
    plt.tight_layout()
    plt.savefig(img_dir + '/xy_target.' + ext)


if __name__ == '__main__':
    # Initialize calculator
    calculator = CpuCalculator()

    # Initialize sound sources
    calculator.init_wave_sources(NUM_TRANS_X * NUM_TRANS_Y)

    center = np.array([TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0, Z])

    # initialize position, direction, amplitude and phase of each sound source
    wave_sources = calculator.wave_sources()
    for y in range(NUM_TRANS_Y):
        for x in range(NUM_TRANS_X):
            pos = np.array([TRANS_SIZE * x, TRANS_SIZE * y, 0.])

            i = x + y * NUM_TRANS_X
            wave_sources[i].pos = pos
            wave_sources[i].amp = 0.0
            wave_sources[i].phase = 0.0

    # SMILE
    radius = 45.0
    num = 30
    target_pos = []
    for i in range(num):
        theta = 2 * math.pi * i / num
        target_pos.append(center + radius * np.array([math.cos(theta), math.sin(theta), 0.0]))
    target_pos.append(center + np.array([radius * 0.3, radius * 0.3, 0]))
    target_pos.append(center + np.array([-radius * 0.3, radius * 0.3, 0]))
    for i in range(1, num // 4):
        theta = -math.pi * i / (num // 4)
        target_pos.append(center + radius * 0.6 * np.array([math.cos(theta), math.sin(theta), 0.0]))
    amps = 1.0 * np.ones(len(target_pos))

    print('target amp: ', amps[0]**2)

    setup_pyplot()
    ext = 'pdf'

    img_dir = 'img'
    os.makedirs(img_dir, exist_ok=True)

    plot_target_xy(target_pos, amps, img_dir, ext=ext)

    # ######### GHR-BF #####################
    optimizer = Optimizer.greedy_brute_force(calculator, target_pos, amps)
    plot_phase_xy(wave_sources, img_dir + '/gbs', ext=ext)

    # ######## HORN #####################
    optimizer = Optimizer.horn(calculator, target_pos, amps, 1000, 1e-3, 0.9)
    plot_phase_xy(wave_sources, img_dir + '/horn', ext=ext)

    # ######## Long #####################
    optimizer = Optimizer.long2014(calculator, target_pos, amps, 1.0)
    plot_phase_xy(wave_sources, img_dir + '/long', ext=ext)

    # ####### Levenberg Marquardt #####################
    Optimizer.levenberg_marquardt(calculator, target_pos, amps)
    plot_phase_xy(wave_sources, img_dir + '/lm', ext=ext)

    # ####### GS-PAT #####################
    Optimizer.gspat(calculator, target_pos, amps)
    plot_phase_xy(wave_sources, img_dir + '/gspat', ext=ext)
