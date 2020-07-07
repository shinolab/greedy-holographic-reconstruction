#! /bin/sh

sudo apt update
sudo apt install -y build-essential gfortran
mkdir tmp
cd tmp
wget https://github.com/Reference-LAPACK/lapack/archive/v3.9.0.tar.gz
tar zxvf v3.9.0.tar.gz
cd lapack-3.9.0
cp INSTALL/make.inc.gfortran ./make.inc
cd BLAS
make
cd ..
cd CBLAS
make
cd ..
make
cd LAPACKE
make
cd ..
sudo cp *.a /usr/local/lib
sudo cp CBLAS/include/*.h /usr/local/include
sudo cp LAPACKE/include/*.h /usr/local/include

cd ..
sudo rm -rf tmp
