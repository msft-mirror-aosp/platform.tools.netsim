// Copyright 2023 The Android Open Source Project
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

#include <cstdint>
#include <memory>
#include <vector>

#include "util/log.h"

namespace netsim::wifi {

void handle_wifi_request(uint32_t facade_id,
                         const std::shared_ptr<std::vector<uint8_t>> &packet) {
  BtsLog("netsim::wifi::handle_wifi_request()");
}

}  // namespace netsim::wifi
