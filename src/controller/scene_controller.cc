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

#include <cmath>
#include <cstddef>

#include "controller/device_notify_manager.h"

namespace netsim {
namespace controller {

/* static */
SceneController &SceneController::Singleton() {
  static SceneController *kInstance = new SceneController();
  return *kInstance;
}

void SceneController::Add(std::shared_ptr<Device> &device) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  devices_.push_back(device);
}

const std::vector<std::shared_ptr<Device>> SceneController::Copy() {
  std::unique_lock<std::mutex> lock(this->mutex_);
  return devices_;
}

bool SceneController::SetPosition(const std::string &device_serial,
                                  const netsim::model::Position &position) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  for (auto &device : devices_) {
    if (device->model.device_serial() == device_serial) {
      device->model.mutable_position()->CopyFrom(position);
      DeviceNotifyManager::Get().Notify();
      return true;
    }
  }
  return false;
}

std::shared_ptr<Device> SceneController::GetOrCreate(
    const std::string &serial) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  auto device = GetDevice(serial);
  if (device != nullptr) {
    return device;
  }
  device = std::make_shared<Device>(serial);
  devices_.push_back(device);
  return device;
}

std::shared_ptr<Device> SceneController::GetDevice(const std::string &serial) {
  for (auto device : devices_) {
    if (device->model.device_serial() == serial) return device;
  }
  return {nullptr};
}

// Returns a Device shared_ptr or nullptr
std::shared_ptr<Device> SceneController::MatchDevice(const std::string &serial,
                                                     const std::string &name) {
  std::shared_ptr<Device> found = nullptr;
  if (serial.empty() && name.empty()) {
    return nullptr;
  }
  for (auto &device : devices_) {
    // serial && name -> rename, only match by serial
    // serial && !name -> match by serial
    // !serial && name -> match by name
    auto pos = (serial.empty()) ? device->model.name().find(name)
                                : device->model.device_serial().find(serial);
    if (pos != std::string::npos) {
      // check for multiple matches
      if (found != nullptr) return nullptr;
      found = device;
    }
  }
  return found;
}

// UI requesting a change in device info
bool SceneController::UpdateDevice(const netsim::model::Device &request) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  if (request.device_serial().empty()) {
    return false;
  }
  auto device = MatchDevice(request.device_serial(), request.name());
  if (device == nullptr) return false;
  device->Update(request);
  DeviceNotifyManager::Get().Notify();
  return true;
}

// Euclidian distance between two devices.
float SceneController::GetDistance(const Device &a, const Device &b) {
  return sqrt((pow(a.model.position().x() - b.model.position().x(), 2) +
               pow(a.model.position().y() - b.model.position().y(), 2) +
               pow(a.model.position().z() - b.model.position().z(), 2)));
}

}  // namespace controller
}  // namespace netsim
