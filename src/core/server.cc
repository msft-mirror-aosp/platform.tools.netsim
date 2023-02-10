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

#include "core/server.h"

#include <memory>
#include <thread>

#ifdef NETSIM_ANDROID_EMULATOR
#include "backend/backend_server.h"
#endif
#include "frontend/frontend_server.h"
#include "grpcpp/security/server_credentials.h"
#include "grpcpp/server.h"
#include "grpcpp/server_builder.h"
#include "netsim_cxx_generated.h"
#include "util/filesystem.h"
#include "util/ini_file.h"
#include "util/log.h"
#include "util/os_utils.h"

namespace netsim::server {

namespace {
std::pair<std::unique_ptr<grpc::Server>, std::string> RunGrpcServer() {
  grpc::ServerBuilder builder;
  int selected_port;
  builder.AddListeningPort("0.0.0.0:0", grpc::InsecureServerCredentials(),
                           &selected_port);
  static auto frontend_service = GetFrontendService();
  builder.RegisterService(frontend_service.get());
#ifdef NETSIM_ANDROID_EMULATOR
  static auto backend_service = GetBackendService();
  builder.RegisterService(backend_service.get());
#endif
  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());

  BtsLog("Grpc server listening on localhost: %s",
         std::to_string(selected_port).c_str());

  // Writes grpc port to ini file.
  auto filepath = osutils::GetDiscoveryDirectory() + netsim::filesystem::slash +
                  "netsim.ini";
  IniFile iniFile(filepath);
  iniFile.Read();
  iniFile.Set("grpc.port", std::to_string(selected_port));
  iniFile.Write();

  return std::make_pair(std::move(server), std::to_string(selected_port));
}
}  // namespace

void Run() {
  // Run frontend and backend grpc servers.
  auto [grpc_server, grpc_port] = RunGrpcServer();
  // Run frontend http server.
  std::thread frontend_http_server(RunFrontendHttpServer);

  grpc_server->Wait();
  frontend_http_server.join();
  // never happens
}

}  // namespace netsim::server
