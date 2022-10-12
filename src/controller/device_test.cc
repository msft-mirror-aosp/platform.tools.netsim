// Copyright 2022 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "controller/device.h"

#include "gtest/gtest.h"
#include "model.pb.h"

namespace netsim {
namespace testing {
namespace {

TEST(DeviceTest, CreateDevicesTest) {
  const auto device = netsim::controller::CreateDevice("test-device-serial");
  EXPECT_EQ(device->model.device_serial(), "test-device-serial");
}

}  // namespace
}  // namespace testing
}  // namespace netsim
