/*
 * Copyright 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>
#include <string_view>

#include "model.pb.h"

namespace netsim {
namespace controller {

class Device {
 public:
  model::Device model;

  // virtual void Update(const model::Device& request);

  explicit Device(const model::Device &model) : model(model) {}
};

std::shared_ptr<Device> CreateDevice(std::string_view device_serial);

}  // namespace controller
}  // namespace netsim
