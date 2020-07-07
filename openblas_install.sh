#! /bin/sh

sudo apt update
sudo apt install -y build-essential git python-dev gfortran
mkdir tmp
cd tmp
git clone https://github.com/xianyi/OpenBLAS
cd OpenBLAS
make FC=gfortran
sudo make PREFIX=/opt/openblas install
cd ../..
sudo rm -rf tmp
