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

#!/bin/bash
#
# Setup cmake for netsim.
REPO=$(dirname "$0")/../../..
# Create all the symlinks for rust crates.
$REPO/external/qemu/android/rebuild.sh --task CratePrepare
# Runs the CMake Ninja generator.
$REPO/external/qemu/android/rebuild.sh --task Configure --cmake_option CMAKE_EXPORT_COMPILE_COMMANDS=1
