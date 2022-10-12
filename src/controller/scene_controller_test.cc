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

#include "controller/scene_controller.h"

#include "controller/device.h"
#include "gtest/gtest.h"
#include "model.pb.h"

namespace netsim {
namespace controller {

class SceneControllerTest : public ::testing::Test {
 protected:
  std::shared_ptr<Device> match(const std::string &serial,
                                const std::string &name) {
    return SceneController::Singleton().MatchDevice(serial, name);
  }
};

TEST_F(SceneControllerTest, GetTest) {
  const auto size = SceneController::Singleton().Copy().size();
  EXPECT_EQ(size, 0);
}

TEST_F(SceneControllerTest, AddDevicesAndGetTest) {
  auto device = controller::CreateDevice("a");
  netsim::controller::SceneController::Singleton().Add(device);
  const auto size = SceneController::Singleton().Copy().size();
  EXPECT_EQ(size, 1);
}

TEST_F(SceneControllerTest, DeviceConstructorTest) {
  auto device = controller::CreateDevice("unique-serial");
  EXPECT_EQ("unique-serial", device->model.device_serial());
  // Test for non-empty position and orientationa
  EXPECT_TRUE(device->model.has_position());
  EXPECT_TRUE(device->model.has_orientation());
}

TEST_F(SceneControllerTest, MatchDeviceTest) {
  auto device = controller::CreateDevice("serial:aaa");
  device->model.set_name("name:bbb");
  SceneController::Singleton().Add(device);

  device = controller::CreateDevice("serial:ccc");
  device->model.set_name("name:ddd");
  SceneController::Singleton().Add(device);

  // with both serial and name, uses serial since name won't match
  ASSERT_TRUE(match("aa", "ee") != nullptr);

  // with neither matching
  ASSERT_TRUE(match("ff", "ee") == nullptr);

  // with no serial, matches with name
  ASSERT_TRUE(match("", "dd") != nullptr);
}

}  // namespace controller
}  // namespace netsim
