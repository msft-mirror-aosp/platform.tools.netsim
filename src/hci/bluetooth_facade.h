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
#include <cstdint>
#include <memory>
#include <string>

#include "hci/rust_device.h"
#include "netsim/model.pb.h"
#include "rust/cxx.h"

/** Manages the bluetooth chip emulation provided by the root canal library.
 *
 * Owns the TestModel, setup, and manages the packet flow into and out of
 * rootcanal.
 */

namespace netsim::hci::facade {
// Use forward declaration instead of including "netsim-cxx/src/lib.rs.h".
struct DynRustBluetoothChipCallbacks;
struct AddRustDeviceResult;

void Reset(uint32_t);
void Remove(uint32_t);
void Patch(uint32_t, const model::Chip::Bluetooth &);
model::Chip::Bluetooth Get(uint32_t);
uint32_t Add(uint32_t simulation_device, const std::string &address_string,
             const rust::Slice<::std::uint8_t const> controller_proto_bytes);

rust::Box<AddRustDeviceResult> AddRustDevice(
    uint32_t simulation_device,
    rust::Box<DynRustBluetoothChipCallbacks> callbacks, const std::string &type,
    const std::string &address);

void RemoveRustDevice(uint32_t facade_id);

void Start(uint16_t instance_num);
void Stop();

// Cxx functions for rust ffi.
void PatchCxx(uint32_t id, const rust::Slice<::std::uint8_t const> proto_bytes);
rust::Vec<::std::uint8_t> GetCxx(uint32_t id);

}  // namespace netsim::hci::facade
