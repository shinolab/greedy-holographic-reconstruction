# Greedy Holographic Reconstruction

These programs rely on OpenBLAS.

# Install OpenBLAS

* The programs require that OpenBLAS has been installed in the following way, otherwise, you must specify OpenBLAS path in `build.rs`.

## Linux

* Use `openblas_install.sh`
* Then add `/opt/openblas/lib` to `LD_LIBRARY_PATH` (e.g. add `export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/opt/openblas/lib"` in ~/.bashrc or etc.)

## macOS

* Install via homebrew
    ```
    brew install openblas
    ```
* If you use M1 mac, you must use x86_64 binaries on Rosetta 2.

## Windows

* Install `Visual Studio 2019` ans `Anaconda`, then open `Anaconda Prompt`
    ```
    git clone https://github.com/xianyi/OpenBLAS
    cd OpenBLAS
    conda update -n base conda
    conda config --add channels conda-forge
    conda install -y cmake flang clangdev perl libflang ninja
    "c:/Program Files (x86)/Microsoft Visual Studio/2019/Community/VC/Auxiliary/Build/vcvars64.bat"
    set "LIB=%CONDA_PREFIX%\Library\lib;%LIB%"
    set "CPATH=%CONDA_PREFIX%\Library\include;%CPATH%"
    mkdir build
    cd build
    cmake .. -G "Ninja" -DCMAKE_CXX_COMPILER=clang-cl -DCMAKE_C_COMPILER=clang-cl -DCMAKE_Fortran_COMPILER=flang -DBUILD_WITHOUT_LAPACK=no -DNOFORTRAN=0 -DDYNAMIC_ARCH=ON -DCMAKE_BUILD_TYPE=Release
    cmake --build . --config Release
    cmake --install . --prefix c:\opt -v
    ```

* Then, add environment variable `CONDA_HOME` and set Anaconda home directory path to link `flangmain.lib`.

* If you get `LINK : fatal error LNK1181: cannot open input file 'gfortran.lib'`, please copy `win/gfortran.lib` to `C:\opt\lib\`.
    * This file is just empty lib. `openblas-src` doesn't seem to actually use gfortran, but requires `gfortran.lib`. So, just link the empty lib file.

* See also [OpenBLAS official instruction](https://github.com/xianyi/OpenBLAS/wiki/How-to-use-OpenBLAS-in-Microsoft-Visual-Studio). 

# Author

Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp), 2020-
