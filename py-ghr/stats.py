'''
File: stats.py
Project: py-ghr
Created Date: 09/03/2021
Author: Shun Suzuki
-----
Last Modified: 09/03/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2021 Hapis Lab. All rights reserved.

'''

from scipy import stats
import glob
import re
import os
import pandas as pd


methods = {'horn': 'SDP+BCD', 'long': 'EVD', 'lm': 'LM', 'gspat': 'GS-PAT',
           'gbf_256_16': r'Proposed $(K=16, L=256)$', 'gbf_16_1': r'Proposed $(K=1, L=16)$'}

Ms = [2, 4, 8, 16, 32, 64]


def relative_error():
    data_folder_path = '../ghr/relative_errors'

    r = re.compile(r'(.+)_M(.+)\.csv')

    for m in Ms:
        data_files = []
        for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
            match = r.match(os.path.basename(file))
            if match is None:
                continue
            g = match.groups()
            if len(g) == 2 and int(g[1]) == m:
                data_files.append(file)

        results = pd.DataFrame(columns=methods.keys(), index=methods.keys())
        for file1 in data_files:
            df1 = pd.read_csv(file1, header=None)
            for file2 in data_files:
                df2 = pd.read_csv(file2, header=None)

                g1 = r.match(os.path.basename(file1)).groups()
                g2 = r.match(os.path.basename(file2)).groups()

                res = stats.ttest_ind(df1[0], df2[0], equal_var=False)
                results[g1[0]][g2[0]] = res.pvalue

        print(m)
        pd.options.display.float_format = '{:.3f}'.format
        print(results)


def var():
    data_folder_path = '../ghr/relative_errors'

    r = re.compile(r'(.+)_M(.+)\.csv')

    for m in Ms:
        data_files = []
        for file in glob.glob(os.path.join(data_folder_path, '*.csv')):
            match = r.match(os.path.basename(file))
            if match is None:
                continue
            g = match.groups()
            if len(g) == 2 and int(g[1]) == m:
                data_files.append(file)

        results = pd.DataFrame(columns=methods.keys(), index=methods.keys())
        for file1 in data_files:
            df1 = pd.read_csv(file1, header=None)
            for file2 in data_files:
                df2 = pd.read_csv(file2, header=None)

                g1 = r.match(os.path.basename(file1)).groups()
                g2 = r.match(os.path.basename(file2)).groups()

                res = stats.ttest_ind(df1[1], df2[1], equal_var=False)
                results[g1[0]][g2[0]] = res.pvalue

        print(m)
        pd.options.display.float_format = '{:.3f}'.format
        print(results)


if __name__ == '__main__':
    # relative_error()
    var()
