#!/usr/bin/env bash

# Copyright 2024 The Android Open Source Project
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

# Build and update Rust bindings for libslirp-rs on netsim-dev branch

# The script is necessary because Clang binary, used for creating bindings,
# isn't included in prebuilts for all platforms.

# Prerequisites (Linux/macOS):
# Install Clang to use bindgen: https://rust-lang.github.io/rust-bindgen/requirements.html

# Instructions (Linux/macOS):
# Run this script manually to regenerate all bindings.rs files.

# Windows instructions:
# 1. Install official pre-built LLVM binary:
#    https://rust-lang.github.io/rust-bindgen/requirements.html
# 2. Set the `LIBCLANG_PATH` environment variable to point to the 'bin'
#    directory within your LLVM installation:
#    `set LIBCLANG_PATH=C:\Program Files\LLVM\bin`
# 3. Uncomment the lines starting with `##` in Cargo.toml.
# 4. In rust/daemon/Cargo.toml, update `pica` version to "0.1.7"
#    (version "0.1.9" is unavailable in crates.io).
# 5. Navigate to the rust/libslirp-rs directory and run `cargo build`.
# 6. Revert the change in Cargo.toml: `git checkout rust/libslirp-rs/Cargo.toml`

# Absolute path to tools/netsim using this scripts directory
REPO_NETSIM=$(dirname $(readlink -f "$0"))/..
echo "REPO_NETSIM: ${REPO_NETSIM}"
CARGO_MANIFEST=$REPO_NETSIM/rust/libslirp-rs/Cargo.toml

# Uncomment lines starting with `##`
OS=$(uname | tr '[:upper:]' '[:lower:]')
if [[ "$OS" == "linux" ]]; then
    sed -i 's/^##//g' $CARGO_MANIFEST
fi
if [[ "$OS" == "darwin" ]]; then
    sed -i '' 's/^##//g' $CARGO_MANIFEST
fi

if [ ! -d "$REPO_NETSIM/objs/rust/.cargo" ]; then
    python3 $REPO_NETSIM/scripts/build_tools.py
fi

# Use Rust dependency crates available on netsim-dev branch
export CARGO_HOME=$REPO_NETSIM/objs/rust/.cargo

cd $REPO_NETSIM
cargo build --manifest-path $CARGO_MANIFEST

# Undo changed to Cargo.toml
git checkout $CARGO_MANIFEST

rm $REPO_NETSIM/rust/Cargo.lock
