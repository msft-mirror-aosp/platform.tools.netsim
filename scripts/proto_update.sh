#!/usr/bin/env bash

# Copyright 2022 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Update the Rust protobufs on netsim-dev branch
#
# scripts/build_tools.sh
# ninja -C objs netsimd
# repo start proto-update
# scripts/proto_update.sh
# git add rust/proto
#
# You may need to install protobuf-compiler
#
# Linux: sudo apt-get install protobuf-compiler
# Mac:   brew install protobuf

# Absolute path to tools/netsim using this scripts directory
REPO=$(dirname $(readlink -f "$0"))/..
CARGO=$REPO/rust/proto/Cargo.toml


# run protoc command to generate grpc proto rust files
# Can not generate files by proto/build.rs because protoc-grpcio doesn't support protobuf v3 yet.
# https://github.com/mtp401/protoc-grpcio/issues/41

# Install compilers since the crates are not in AOSP
# TODO: Add required crate mappings to work in netsim-dev
export CARGO_HOME=""
# Specify versions to use the correct protobuf version.
cargo install protobuf-codegen --version 3.2.0
cargo install grpcio-compiler --version 0.13.0

PROTOC_CMD="protoc --rust_out=./rust/proto/src --grpc_out=./rust/proto/src\
 --plugin=protoc-gen-grpc=`which grpc_rust_plugin`\
 -I./proto -I../../external/protobuf/src\
 -I../../packages/modules/Bluetooth/tools/rootcanal/proto"
$PROTOC_CMD ./proto/netsim/frontend.proto
$PROTOC_CMD ./proto/netsim/packet_streamer.proto

# Revert the generate proto files because they will be generatd by proto/build.rs.
git checkout $REPO/rust/proto/src/packet_streamer.rs
git checkout $REPO/rust/proto/src/frontend.rs
rm $REPO/rust/proto/src/mod.rs

# uncomment out lines
sed -i 's/^##//g' $CARGO

# depends on emu-master-dev branch
export CARGO_HOME=$REPO/objs/rust/.cargo

cd $REPO
cargo build --manifest-path $CARGO

# Undo changed to Cargo.toml
git checkout $CARGO

# The possible values are "linux" and "darwin".
OS=$(uname | tr '[:upper:]' '[:lower:]')

# Find the most recent rustfmt installed
RUSTFMT=`ls -d ../../prebuilts/rust/$OS-x86/*/bin/rustfmt | tail -1`

# Format rust code
find $REPO/rust/proto -name '*.rs' -exec $RUSTFMT --files-with-diff {} \;

rm rust/Cargo.lock
