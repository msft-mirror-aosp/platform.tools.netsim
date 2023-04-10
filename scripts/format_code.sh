#!/bin/bash
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

# Formats source files according to Google's style guide.

REPO=$(dirname "$0")/../../..

# Run clang-format.
find $REPO/tools/netsim/src \( -name '*.cc' -o -name '*.h' -o -name '*.proto' \) \
  -exec clang-format -i {} \;

# Format rust.
find $REPO/tools/netsim/rust \( \
  -path $REPO/tools/netsim/rust/target -prune -false \
  -o -name '*.rs' \) \
  -exec $REPO/prebuilts/rust/linux-x86/stable/rustfmt -v {} \;

# Format TypeScript.
find $REPO/tools/netsim/ui/ts \( -name '*.ts' \) \
  -exec clang-format -i {} \;

# Run cmake-format.
find $REPO/tools/netsim \( -name 'CMakeLists.txt' \) \
  -exec cmake-format -i {} \;
find $REPO/tools/netsim/cmake \( -name "*.cmake" \) \
  -exec cmake-format -i {} \;
