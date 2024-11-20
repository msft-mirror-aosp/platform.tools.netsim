#!/bin/bash
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

# This script sets up the necessary environment variables for Cargo builds.
# It determines the OUT_PATH, sets up CARGO_HOME and library paths,
# and defines paths to prebuilt packet files.

# Usage: scripts/cargo_env.sh [OUT_PATH]
#   OUT_PATH: Optional. The output directory for build artifacts.
#             Defaults to "tools/netsim/objs" if not specified.

# Get the directory of the script
REPO=$(dirname "$0")/../../..

# Determine the OUT_PATH
OUT_PATH="${1:-$REPO/tools/netsim/objs}"

# Get OS name (lowercase)
OS=$(uname | tr '[:upper:]' '[:lower:]')

# Set environment variables
export CARGO_HOME=$OUT_PATH/rust/.cargo
export OBJS_PATH=$OUT_PATH

# Paths to pdl generated packets files
ROOTCANAL_PDL_PATH=$OUT_PATH/rootcanal/pdl_gen
export LINK_LAYER_PACKETS_PREBUILT=$ROOTCANAL_PDL_PATH/link_layer_packets.rs
PDL_PATH=$OUT_PATH/pdl/pdl_gen
export MAC80211_HWSIM_PACKETS_PREBUILT=$PDL_PATH/mac80211_hwsim_packets.rs
export IEEE80211_PACKETS_PREBUILT=$PDL_PATH/ieee80211_packets.rs
export LLC_PACKETS_PREBUILT=$PDL_PATH/llc_packets.rs
export NETLINK_PACKETS_PREBUILT=$PDL_PATH/netlink_packets.rs

# Set library path based on OS
if [[ "$OS" == "darwin" ]]; then
  export DYLD_FALLBACK_LIBRARY_PATH=$OUT_PATH/lib64
else
  export LD_LIBRARY_PATH=$OUT_PATH/lib64
fi
