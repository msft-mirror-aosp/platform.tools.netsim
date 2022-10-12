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
#include "hci/hci_chip_emulator.h"

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
// Common update to Phy states used by frontend and Chip Facades
void UpdateRadioInternal(netsim::model::Device &device,
                         const netsim::model::ChipPhyState &radio_state,
                         bool insert) {
  // Phys are stored as in a table, devices have a small number
  for (auto mutable_radio_state : *device.mutable_radio_states()) {
    if (mutable_radio_state.radio() == radio_state.radio()) {
      mutable_radio_state.set_state(radio_state.state());
      return;
    }
  }
  if (insert) {
    // The radio was not found, add a new radio state if called
    // from the Chip Facade.
    auto rs = device.mutable_radio_states()->Add();
    rs->CopyFrom(radio_state);
  }
}

// Internal from Radio Facade
void SceneController::UpdateRadio(const std::string &device_serial,
                                  netsim::model::PhyKind radio,
                                  netsim::model::PhyState state) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  netsim::model::ChipPhyState radio_state;
  radio_state.set_radio(radio);
  radio_state.set_state(state);
  for (auto &device : devices_) {
    if (device->model.device_serial() == device_serial) {
      UpdateRadioInternal(device->model, radio_state, true);
      break;
    }
  }
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
bool SceneController::UpdateDevice(
    const netsim::model::Device &updated_device) {
  std::unique_lock<std::mutex> lock(this->mutex_);
  if (updated_device.device_serial().empty()) {
    return false;
  }
  auto device =
      MatchDevice(updated_device.device_serial(), updated_device.name());
  if (device == nullptr) return false;
  if (updated_device.has_position()) {
    device->model.mutable_position()->CopyFrom(updated_device.position());
  }
  if (updated_device.has_orientation()) {
    device->model.mutable_orientation()->CopyFrom(updated_device.orientation());
  }
  if (!updated_device.name().empty()) {
    device->model.set_name(updated_device.name());
  }
  if (updated_device.radio_states().size()) {
    for (auto &radio_state : updated_device.radio_states()) {
      hci::ChipEmulator::Get().SetDeviceRadio(device->model.device_serial(),
                                              radio_state.radio(),
                                              radio_state.state());
      UpdateRadioInternal(device->model, radio_state, false);
    }
  }
  if (updated_device.radio_ranges().size()) {
    // TODO push to model and chip emulators
  }
  DeviceNotifyManager::Get().Notify();
  return true;
}

std::optional<float> SceneController::GetDistance(
    const std::string &device_serial_a, const std::string &device_serial_b) {
  auto a = GetDevice(device_serial_a);
  if (a == nullptr) return {};
  auto b = GetDevice(device_serial_b);
  if (b == nullptr) return {};
  return sqrt((pow(a->model.position().x() - b->model.position().x(), 2) +
               pow(a->model.position().y() - b->model.position().y(), 2) +
               pow(a->model.position().z() - b->model.position().z(), 2)));
}

}  // namespace controller
}  // namespace netsim
