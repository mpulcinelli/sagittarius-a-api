# TOKEN GIT - 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

LAMBDA_ARCH="linux/amd64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.
# aarch64-unknown-linux-gnu 
# x86_64-unknown-linux-musl
# corresponding with the above, set this to aarch64 or x86_64-unknown-linux-gnu for ARM or x86 functions.
RUST_TARGET="x86_64-unknown-linux-musl"
RUST_VERSION="latest" # Set this to a specific version of rust you want to compile for, or to latest if you want the latest stable version.
PROJECT_NAME="sagittarius-a-api"

sag_build_bin(){
    rm -r -f ./target && cargo build --release --target ${RUST_TARGET} # This line can be any cargo command
}

sag_zip_bin(){
    rm -f lambda.zip && cp ./target/${RUST_TARGET}/release/sagittarius-a-main ./bootstrap  && zip lambda.zip bootstrap && rm bootstrap
}

sag_aws_upload_lambda(){
    aws lambda update-function-code --function-name sagittarius-a-user --zip-file fileb://lambda.zip --output json && rm -f lambda.zip
}

sag_full_deploy(){
    sag_build_bin
    sag_zip_bin
    sag_aws_upload_lambda
}

setupMuslLibs(){
    rustup target install --toolchain stable-x86_64-unknown-linux-gnu  x86_64-unknown-linux-musl
    apt-get install cmake musl-tools clang libc++-dev build-essential autoconf libtool pkg-config 
}