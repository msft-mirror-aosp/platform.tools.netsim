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

#include <chrono>
#include <cmath>
#include <cstddef>
#include <optional>

#include "controller/device_notify_manager.h"
#include "netsim-cxx/src/lib.rs.h"
#include "util/log.h"

namespace netsim {
namespace controller {
namespace {
constexpr std::chrono::seconds kInactiveLimitToShutdown(300);
}

/* static */
SceneController &SceneController::Singleton() {
  static SceneController *kInstance = new SceneController();
  return *kInstance;
}

model::Scene SceneController::Get() {
  model::Scene scene;
  for (auto &[_, device] : devices_) {
    scene.add_devices()->CopyFrom(device->Get());
  }
  return scene;
}

std::tuple<uint32_t, uint32_t, uint32_t> SceneController::AddChip(
    const std::string &guid, const std::string &device_name,
    common::ChipKind chip_kind, const std::string &chip_name,
    const std::string &manufacturer, const std::string &product_name) {
  std::string chip_kind_string;
  switch (chip_kind) {
    case common::ChipKind::BLUETOOTH:
      chip_kind_string = "BLUETOOTH";
      break;
    case common::ChipKind::WIFI:
      chip_kind_string = "WIFI";
      break;
    case common::ChipKind::UWB:
      chip_kind_string = "UWB";
      break;
    default:
      chip_kind_string = "UNSPECIFIED";
      break;
  }
  std::unique_ptr<scene_controller::AddChipResult> add_chip_result_ptr =
      netsim::device::AddChipCxx(guid, device_name, chip_kind_string, chip_name,
                                 manufacturer, product_name);
  uint32_t device_id = add_chip_result_ptr->device_id;
  uint32_t chip_id = add_chip_result_ptr->chip_id;
  uint32_t facade_id = add_chip_result_ptr->facade_id;
  return {device_id, chip_id, facade_id};
}

std::shared_ptr<Device> SceneController::GetDevice(const std::string &guid,
                                                   const std::string &name) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  for (auto &[_, device] : devices_) {
    if (device->guid == guid) return device;
  }
  static uint32_t identifier = 0;
  auto device = std::make_shared<Device>(identifier, guid, name);
  devices_[identifier++] = device;
  return device;
}

void SceneController::RemoveDevice(uint32_t id) {
  if (devices_.find(id) != devices_.end()) {
    auto device = devices_[id];
    BtsLog("SceneController::RemoveDevice - removing %s", device->name.c_str());
    device->Remove();
    devices_.erase(id);
  } else {
    BtsLog("Device not found in remove %d", id);
  }
}

void SceneController::RemoveChip(uint32_t device_id, uint32_t chip_id) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  netsim::device::RemoveChipCxx(device_id, chip_id);
}

// Returns a Device shared_ptr or nullptr
std::shared_ptr<Device> SceneController::MatchDevice(const std::string &name) {
  std::shared_ptr<Device> target = nullptr;
  bool multiple_matches = false;
  if (name.empty()) {
    return nullptr;
  }
  for (auto &[_, device] : devices_) {
    // match by device name
    auto pos = device->name.find(name);
    if (pos != std::string::npos) {
      // check for exact match
      if (device->name == name) return device;
      // record if multiple ambiguous matches were found
      if (target != nullptr) multiple_matches = true;
      target = device;
    }
  }
  // return nullptr if multiple ambiguous matches were found
  if (multiple_matches) return nullptr;
  return target;
}

// UI requesting a change in device info
bool SceneController::PatchDevice(const model::Device &request) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  if (request.name().empty()) {
    return false;
  }
  auto device = MatchDevice(request.name());
  if (device == nullptr) return false;
  device->Patch(request);
  DeviceNotifyManager::Get().Notify();
  return true;
}

// Euclidian distance between two devices.
float SceneController::GetDistance(uint32_t id, uint32_t other_id) {
  return netsim::device::GetDistanceCxx(id, other_id);
}

void SceneController::Reset() {
  std::unique_lock<std::mutex> lock(this->mutex_);
  for (auto &[_, device] : devices_) {
    device->Reset();
  }
  DeviceNotifyManager::Get().Notify();
}

std::optional<std::chrono::seconds> SceneController::GetShutdownTime() {
  if (!inactive_timestamp_.has_value()) return std::nullopt;

  auto inactive_seconds = std::chrono::duration_cast<std::chrono::seconds>(
      std::chrono::system_clock::now() - inactive_timestamp_.value());
  auto remaining_seconds = kInactiveLimitToShutdown - inactive_seconds;
  return std::optional<std::chrono::seconds>(remaining_seconds);
}
}  // namespace controller
}  // namespace netsim
