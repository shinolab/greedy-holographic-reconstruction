#! /bin/bash

pushd ../ghr-capi
cargo build --release
popd

mkdir -p ghr/bin
cp -f ../ghr-capi/target/release/libghrcapi.so ghr/bin/
