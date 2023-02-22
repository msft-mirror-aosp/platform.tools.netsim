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

// Use gRPC HCI PacketType definitions so we don't expose Rootcanal's version
// outside of the Bluetooth Facade.
#include "packet_streamer.pb.h"

namespace netsim {
namespace backend {
namespace grpc {

/* Handle packet requests for the backend. */

void handle_bt_response(uint32_t facade_id,
                        packet::HCIPacket_PacketType packet_type,
                        const std::shared_ptr<std::vector<uint8_t>> packet);

}  // namespace grpc
}  // namespace backend
}  // namespace netsim
