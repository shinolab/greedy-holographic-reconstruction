'''
File: analysis.py
Project: py-ghr
Created Date: 25/01/2021
Author: Shun Suzuki
-----
Last Modified: 25/01/2021
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
    plt.rcParams["legend.frameon"] = False


def relative_error():
    data_folder_path = '../ghr/relative_errors'

    r = re.compile(r'(.+)_M(.+)\.csv')

    methods = ['gbf_16_1', 'gbf_16_16', 'horn', 'long', 'lm', 'gspat']
    foci_nums = set()
    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 2:
            foci_nums.add(int(g[1]))

    data_mean = pd.DataFrame(columns=methods, index=sorted(foci_nums))

    for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
        m = r.match(os.path.basename(file))
        if m is None:
            continue
        g = m.groups()
        if len(g) == 2:
            df = pd.read_csv(file, header=None)
            mean = df[0].mean()
            data_mean[g[0]][int(g[1])] = mean

    print(data_mean)

    fig, ax = plt.subplots()
    bar_width = 0.25
    alpha = 0.8

    for (i, method) in enumerate(methods):
        n = len(data_mean[method].keys())
        index = np.arange(n)
        plt.bar(index + i * bar_width, data_mean[method], bar_width,
                alpha=alpha, label=method)

    plt.ylabel('Counts')
    # plt.xticks(index + bar_width, ('1', '2', '3', '4'))
    plt.legend()
    plt.savefig("yokonarabi_bar.png", dpi=130, bbox_inches='tight', pad_inches=0)
    plt.show()


if __name__ == "__main__":
    relative_error()
    # setup_pyplot()
    # e_mean = pd.read_csv('data/relative_error_mean.csv')
    # e_max = pd.read_csv('data/relative_error_max.csv')
    # e_min = pd.read_csv('data/relative_error_min.csv')

    # m = e_mean['M']
    # gbf = e_mean['GBS']
    # horn = e_mean['HORN']
    # long2014 = e_mean['LONG']
    # lm = e_mean['LM']

    # DPI = 300
    # fig = plt.figure(figsize=(6, 6), dpi=DPI)
    # ax = fig.add_subplot()
    # ax.set_xlabel(r'Number of control points $M$')
    # ax.set_ylabel(r'Relative error [\%]')
    # ax.plot(m, e_mean['GBSk1l16'], label='Proposed ($K=1, L=16$)')
    # ax.fill_between(m, e_min['GBSk1l16'], e_max['GBSk1l16'], alpha=0.5)
    # ax.plot(m, e_mean['GBSk16l256'], label='Proposed ($K=16, L=256$)', linestyle='dashed')
    # ax.fill_between(m, e_min['GBSk16l256'], e_max['GBSk16l256'], alpha=0.5)
    # ax.plot(m, horn, label='SPD+BCD', linestyle='dashdot')
    # ax.fill_between(m, e_min['HORN'], e_max['HORN'], alpha=0.5)
    # ax.plot(m, long2014, label='EVD+Reg.', linestyle='dotted')
    # ax.fill_between(m, e_min['LONG'], e_max['LONG'], alpha=0.5)
    # ax.plot(m, lm, label='LM', linestyle=(10, (5, 3, 1, 3, 1, 3)))
    # ax.fill_between(m, e_min['LM'], e_max['LM'], alpha=0.5)
    # ax.legend(bbox_to_anchor=(1, 0.5), loc='upper right')
    # ax.set_ylim((0, 100))

    # plt.tight_layout()
    # plt.savefig('relative_error.pdf')
    # plt.show()
