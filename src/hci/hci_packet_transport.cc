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

#include "hci/hci_packet_transport.h"

#include <limits>
#include <memory>
#include <optional>

#include "model/hci/hci_transport.h"
#include "netsim-daemon/src/ffi.rs.h"
#include "netsim/hci_packet.pb.h"
#include "rust/cxx.h"
#include "util/log.h"

using netsim::packet::HCIPacket;

namespace netsim {
namespace hci {

std::unordered_map<uint32_t, std::shared_ptr<HciPacketTransport>>
    device_to_transport_;

/**
 * @class HciPacketTransport
 *
 * Connects hci packets between packet_hub and rootcanal.
 *
 */
HciPacketTransport::HciPacketTransport(
    std::shared_ptr<rootcanal::AsyncManager> async_manager)
    : mDeviceId(std::nullopt), mAsyncManager(std::move(async_manager)) {}

/**
 * @brief Connect the phy device to the transport
 *
 * @param - device_id identifier of the owning device
 */
void HciPacketTransport::Connect(rootcanal::PhyDevice::Identifier device_id) {
  assert(!mDeviceId.has_value());
  mDeviceId.emplace(device_id);
}

// Called by HCITransport (rootcanal)
void HciPacketTransport::Send(rootcanal::PacketType packet_type,
                              const std::vector<uint8_t> &data) {
  // The packet types have standard values, converting from
  // rootcanal::PacketType to HCIPacket_PacketType is safe.
  packet::HCIPacket_PacketType hci_packet_type =
      static_cast<packet::HCIPacket_PacketType>(packet_type);
  if (!mDeviceId.has_value()) {
    BtsLogWarn("hci_packet_transport: response with no device.");
    return;
  }
  // Send response to transport dispatcher.
  netsim::echip::HandleResponse(common::ChipKind::BLUETOOTH, mDeviceId.value(),
                                data, hci_packet_type);
}

// Called by HCITransport (rootcanal)
void HciPacketTransport::RegisterCallbacks(
    rootcanal::PacketCallback packetCallback,
    rootcanal::CloseCallback closeCallback) {
  BtsLogInfo("hci_packet_transport: registered");
  mPacketCallback = packetCallback;
  mCloseCallback = closeCallback;
}

// Called by HCITransport (rootcanal)
void HciPacketTransport::Tick() {}

void HciPacketTransport::Request(
    packet::HCIPacket_PacketType packet_type,
    const std::shared_ptr<std::vector<uint8_t>> &packet) {
  assert(mPacketCallback);
  // The packet types have standard values, converting from
  // HCIPacket_PacketType to rootcanal::PacketType is safe.
  rootcanal::PacketType rootcanal_packet_type =
      static_cast<rootcanal::PacketType>(packet_type);
  mAsyncManager->Synchronize([this, rootcanal_packet_type, packet]() {
    mPacketCallback(rootcanal_packet_type, packet);
  });
}

void HciPacketTransport::Add(
    rootcanal::PhyDevice::Identifier device_id,
    const std::shared_ptr<HciPacketTransport> &transport) {
  transport->Connect(device_id);
  device_to_transport_[device_id] = transport;
}

void HciPacketTransport::Remove(rootcanal::PhyDevice::Identifier device_id) {
  BtsLogInfo("hci_packet_transport remove from netsim");
  if (device_to_transport_.find(device_id) != device_to_transport_.end() &&
      device_to_transport_[device_id]) {
    // Calls HciDevice::Close, will disconnect AclHandles with
    // CONNECTION_TIMEOUT, and call TestModel::CloseCallback.
    device_to_transport_[device_id]->mCloseCallback();
  }
}

// Called by HciDevice::Close
void HciPacketTransport::Close() {
  if (mDeviceId.has_value()) {
    device_to_transport_.erase(mDeviceId.value());
  }
  BtsLogInfo("hci_packet_transport close from rootcanal");
  mDeviceId = std::nullopt;
}

// handle_request is the main entry for incoming packets called by
// netsim::packet_hub
//
// Transfer the request to the HciTransport to deliver to Rootcanal via the
// acl/sco/iso/command callback methods under synchronization.
void handle_bt_request(uint32_t facade_id,
                       packet::HCIPacket_PacketType packet_type,
                       const std::shared_ptr<std::vector<uint8_t>> &packet) {
  if (device_to_transport_.find(facade_id) != device_to_transport_.end() &&
      device_to_transport_[facade_id]) {
    auto transport = device_to_transport_[facade_id];
    transport->Request(packet_type, packet);
  } else {
    std::cout << "device_to_transport_ ids ";
    for (auto [k, _] : device_to_transport_) std::cout << k << " ";
    std::cout << std::endl;
    BtsLogWarn(
        "hci_packet_transport: handle_request with no transport for device "
        "with facade_id: %d",
        facade_id);
  }
}

void HandleBtRequestCxx(uint32_t facade_id, uint8_t packet_type,
                        const rust::Vec<uint8_t> &packet) {
  std::vector<uint8_t> buffer(packet.begin(), packet.end());
  auto packet_ptr = std::make_shared<std::vector<uint8_t>>(buffer);
  handle_bt_request(facade_id,
                    static_cast<packet::HCIPacket_PacketType>(packet_type),
                    packet_ptr);
}

}  // namespace hci
}  // namespace netsim
