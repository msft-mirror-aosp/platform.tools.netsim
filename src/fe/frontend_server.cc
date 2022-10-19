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

#include "fe/frontend_server.h"

#include <iostream>
#include <memory>
#include <string>

#include "controller/scene_controller.h"
#include "frontend.grpc.pb.h"
#include "frontend.pb.h"
#include "google/protobuf/empty.pb.h"
#include "grpcpp/security/server_credentials.h"
#include "grpcpp/server.h"
#include "grpcpp/server_builder.h"
#include "grpcpp/server_context.h"
#include "grpcpp/support/status.h"
#include "hci/hci_chip_emulator.h"
#include "util/ini_file.h"
#include "util/os_utils.h"

namespace netsim {

class FrontendServer final : public frontend::FrontendService::Service {
 public:
  grpc::Status GetVersion(grpc::ServerContext *context,
                          const google::protobuf::Empty *empty,
                          frontend::VersionResponse *reply) {
    reply->set_version("123b");
    return grpc::Status::OK;
  }

  grpc::Status GetDevices(grpc::ServerContext *context,
                          const google::protobuf::Empty *empty,
                          frontend::GetDevicesResponse *reply) {
    const auto devices =
        netsim::controller::SceneController::Singleton().Copy();
    for (const auto &device : devices)
      reply->add_devices()->CopyFrom(device->model);
    return grpc::Status::OK;
  }

  grpc::Status SetPosition(grpc::ServerContext *context,
                           const frontend::SetPositionRequest *request,
                           google::protobuf::Empty *empty) {
    auto status = netsim::controller::SceneController::Singleton().SetPosition(
        request->device_serial(), request->position());
    if (!status)
      return grpc::Status(grpc::StatusCode::NOT_FOUND,
                          "device " + request->device_serial() + " not found.");
    return grpc::Status::OK;
  }

  grpc::Status UpdateDevice(grpc::ServerContext *context,
                            const frontend::UpdateDeviceRequest *request,
                            google::protobuf::Empty *response) {
    auto status = netsim::controller::SceneController::Singleton().UpdateDevice(
        request->device());
    if (!status)
      return grpc::Status(
          grpc::StatusCode::NOT_FOUND,
          "device " + request->device().device_serial() + " not found.");
    return grpc::Status::OK;
  }

  grpc::Status SetPacketCapture(
      grpc::ServerContext *context,
      const frontend::SetPacketCaptureRequest *request,
      google::protobuf::Empty *empty) {
    hci::ChipEmulator::Get().SetPacketCapture(request->device_serial(),
                                              request->capture());
    return grpc::Status::OK;
  }
};

void RunFrontendServer() {
  FrontendServer service;

  grpc::ServerBuilder builder;
  int selected_port;
  builder.AddListeningPort("0.0.0.0:0", grpc::InsecureServerCredentials(),
                           &selected_port);
  builder.RegisterService(&service);
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());

  std::cout << "Server listening on localhost:" << selected_port << std::endl;

  // Writes port to ini file.
  auto filepath = osutils::GetDiscoveryDirectory().append("netsim.ini");
  IniFile iniFile(filepath);
  iniFile.Set("grpc.port", std::to_string(selected_port));
  iniFile.Write();

  server->Wait();
}

}  // namespace netsim
