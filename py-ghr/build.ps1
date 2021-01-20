cd ../ghr-capi
cargo build --release
cd ../py-ghr

if (!(Test-Path ghr/bin -PathType Container)) {
    New-Item -ItemType Directory -Force -Path ghr/bin
}
cp -Force ../ghr-capi/target/release/ghrcapi.dll ghr/bin/
