'''
File: analysis.py
Project: py-ghr
Created Date: 25/01/2021
Author: Shun Suzuki
-----
Last Modified: 29/01/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2021 Hapis Lab. All rights reserved.

'''

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import re
import glob
import os

methods = {'horn': 'SDP+BCD', 'long': 'EVD', 'lm': 'LM', 'gspat': 'GS-PAT',
           'gbf_256_16': r'Proposed $(K=16, L=256)$', 'gbf_16_1': r'Proposed $(K=1, L=16)$'}


DPI = 300
ext = 'png'


def setup_pyplot():
    plt.rcParams['text.usetex'] = True
    plt.rcParams['axes.grid'] = False
    plt.rcParams['xtick.direction'] = 'in'
    plt.rcParams['ytick.direction'] = 'in'
    plt.rcParams['xtick.major.width'] = 1.0
    plt.rcParams['ytick.major.width'] = 1.0
    plt.rcParams['font.size'] = 14
    plt.rcParams['font.family'] = 'sans-serif'
    plt.rcParams['font.sans-serif'] = ['Arial']
    plt.rcParams["mathtext.fontset"] = 'stixsans'
    plt.rcParams['ps.useafm'] = True
    plt.rcParams['pdf.use14corefonts'] = True
    plt.rcParams['text.latex.preamble'] = r'\usepackage{sfmath}'
    plt.rcParams["legend.frameon"] = False


def relative_error():
    data_folder_path = '../ghr/relative_errors'

    r = re.compile(r'(.+)_M(.+)\.csv')

    foci_nums = set()
    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 2:
            foci_nums.add(int(g[1]))

    foci_nums = sorted(foci_nums)
    data_mean = pd.DataFrame(columns=methods.keys(), index=foci_nums)
    data_error = pd.DataFrame(columns=methods.keys(), index=foci_nums)

    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 2:
            df = pd.read_csv(file, header=None)
            data_mean[g[0]][int(g[1])] = 100.0 + df[0].mean()
            data_error[g[0]][int(g[1])] = df[0].std()

    #
    fig = plt.figure(figsize=(12, 6), dpi=DPI)
    axes = fig.add_subplot(111)

    bar_width = 0.15
    alpha = 1.0

    n = len(foci_nums)
    index = np.arange(n)
    for (i, (k, v)) in enumerate(methods.items()):
        axes.bar(index + i * bar_width, data_mean[k], bar_width,
                 yerr=data_error[k], capsize=4, alpha=alpha, label=v)

    axes.set_xticks(np.arange(len(foci_nums)) + bar_width * (len(foci_nums) - 1) / 2, minor=False)
    axes.set_yticks(np.arange(0, 120, 20), minor=False)
    x_labels = [foci_nums[i] for i in range(len(foci_nums))]
    y_labels = np.arange(0, 120, 20)
    axes.set_xticklabels(x_labels, minor=False, fontsize=12)
    axes.set_yticklabels(y_labels, minor=False, fontsize=12)
    axes.tick_params(bottom=False, left=True, right=False, top=False)

    axes.hlines([100], -1.5 * bar_width, n, 'black', linestyles='dashed')
    axes.set_xlim((-1.5 * bar_width, n))

    plt.ylabel(r'Accuracy [\%]')
    plt.xlabel(r'Number of control points $M$')
    axes.legend()
    plt.legend(loc='upper center', bbox_to_anchor=(0.5, -0.1), ncol=len(foci_nums), fontsize=12)
    plt.tight_layout()
    plt.savefig('accuracy.' + ext)


def time_foci():
    data_folder_path = '../ghr/times_foci'

    r = re.compile(r'(.+)_M(\d+)_N(\d+)\.csv')

    foci_nums = set()
    trans_num = 0
    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 3:
            foci_nums.add(int(g[1]))
            trans_num = int(g[2])

    foci_nums = sorted(foci_nums)
    data_mean = pd.DataFrame(columns=methods.keys(), index=foci_nums)
    data_error = pd.DataFrame(columns=methods.keys(), index=foci_nums)

    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 3:
            df = pd.read_csv(file, header=None)
            data_mean[g[0]][int(g[1])] = df[0].mean() / 1000
            data_error[g[0]][int(g[1])] = df[0].std() / 1000

    data_mean = data_mean.dropna(axis=1)
    print(data_mean)
    print('trans num', trans_num)

    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    ax = fig.add_subplot()
    ax.set_xlabel(r'Number of control points $M$')
    ax.set_ylabel('Time [ms]')
    ax.set_title('Number of wave sources is ' + str(trans_num))

    linestyles = ['solid', 'dashed', 'dashdot', 'dotted', (10, (5, 3, 1, 3, 1, 3))]
    for (i, k) in enumerate(data_mean):
        ax.errorbar(foci_nums, data_mean[k], yerr=data_error[k], capsize=5, fmt='o', markersize=10)
        ax.plot(foci_nums, data_mean[k], label=methods[k], linestyle=linestyles[i])

    plt.xscale('log')
    plt.yscale('log')
    ticks = [str(m) for m in foci_nums]
    ax.set_xticks(foci_nums)
    ax.set_xticklabels(ticks)
    ax.legend()
    plt.minorticks_off()
    plt.tight_layout()
    plt.savefig('vs_M_log.' + ext)


def time_trans():
    data_folder_path = '../ghr/times_trans'

    r = re.compile(r'(.+)_M(\d+)_N(\d+)\.csv')

    foci_num = 0
    trans_nums = set()
    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 3:
            foci_num = int(g[1])
            trans_nums.add(int(g[2]))

    trans_nums = sorted(trans_nums)
    data_mean = pd.DataFrame(columns=methods.keys(), index=trans_nums)
    data_error = pd.DataFrame(columns=methods.keys(), index=trans_nums)

    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 3:
            df = pd.read_csv(file, header=None)
            data_mean[g[0]][int(g[2])] = df[0].mean()
            data_error[g[0]][int(g[2])] = df[0].std()

    data_mean = data_mean.dropna(axis=1)
    print(data_mean)
    print('num foci', foci_num)

    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    ax = fig.add_subplot()
    ax.set_xlabel(r'Number of control points $M$')
    ax.set_ylabel('Time [ms]')
    ax.set_title('Number of control points is ' + str(foci_num))

    linestyles = ['solid', 'dashed', 'dashdot', 'dotted', (10, (5, 3, 1, 3, 1, 3))]
    for (i, k) in enumerate(data_mean):
        ax.errorbar(trans_nums, data_mean[k], yerr=data_error[k], capsize=5, fmt='o', markersize=10)
        ax.plot(trans_nums, data_mean[k], label=methods[k], linestyle=linestyles[i])

    plt.xscale('log')
    plt.yscale('log')
    ticks = [str(m) for m in trans_nums]
    ax.set_xticks(trans_nums)
    ax.set_xticklabels(ticks)
    ax.legend()
    plt.minorticks_off()
    plt.tight_layout()
    plt.savefig('vs_N_log.' + ext)


if __name__ == "__main__":
    setup_pyplot()

    # relative_error()
    time_foci()
    time_trans()
