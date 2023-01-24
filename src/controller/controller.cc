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

#include "controller/controller.h"

#include <google/protobuf/empty.pb.h>
#include <google/protobuf/util/json_util.h>

#include <memory>
#include <string>

#include "controller/scene_controller.h"
#include "frontend.pb.h"

namespace netsim::scene_controller {

unsigned int UpdateDevice(const std::string &request,
                          std::unique_ptr<std::string> response,
                          std::unique_ptr<std::string> error_message) {
  frontend::UpdateDeviceRequest request_proto;
  google::protobuf::util::JsonStringToMessage(request, &request_proto);
  google::protobuf::Empty response_proto;

  auto status = netsim::controller::SceneController::Singleton().UpdateDevice(
      request_proto.device());
  if (!status) {
    error_message.reset(new std::string(
        "device_serial not found: " + request_proto.device().device_serial()));
    return HTTP_STATUS_BAD_REQUEST;
  }

  google::protobuf::util::MessageToJsonString(response_proto, response.get());
  return HTTP_STATUS_OK;
}

unsigned int GetDevices(const std::string &request,
                        std::unique_ptr<std::string> response,
                        std::unique_ptr<std::string> error_message) {
  frontend::GetDevicesResponse response_proto;
  const auto &devices = netsim::controller::SceneController::Singleton().Copy();
  for (const auto &device : devices)
    response_proto.add_devices()->CopyFrom(device->model);

  google::protobuf::util::MessageToJsonString(response_proto, response.get());
  return HTTP_STATUS_OK;
}

}  // namespace netsim::scene_controller