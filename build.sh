#! /bin/bash

pushd ghr-capi
cargo build --release
popd

cp -f ghr-capi/target/release/libghrcapi.so py-ghr/ghr/bin/
