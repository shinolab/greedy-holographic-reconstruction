#! /bin/bash

pushd ghr-capi
cargo build --release
popd

mkdir -p py-ghr/ghr/bin
cp -f ghr-capi/target/release/libghrcapi.so py-ghr/ghr/bin/
