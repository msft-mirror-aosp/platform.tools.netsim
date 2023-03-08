# Copyright 2022 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

add_subdirectory(proto)

android_add_library(
  TARGET util-lib
  LICENSE Apache-2.0
  SRC util/filesystem.h
      util/ini_file.cc
      util/ini_file.h
      util/log.cc
      util/log.h
      util/os_utils.cc
      util/os_utils.h
      util/string_utils.cc
      util/string_utils.h)
target_include_directories(util-lib PRIVATE .)

android_add_library(
  TARGET core-lib
  LICENSE Apache-2.0
  SRC core/server.cc core/server.h
  DEPS frontend-lib
       grpc++
       netsim-cli-proto-lib
       netsim-cxx-generated
       packet-streamer-server-lib
       protobuf::libprotobuf
       util-lib)

target_include_directories(core-lib PRIVATE . ../rust/netsim-cxx
                                            ../rust/netsim-cxx/cxx)
target_compile_definitions(core-lib PUBLIC NETSIM_ANDROID_EMULATOR)

android_add_library(
  TARGET controller-lib
  LICENSE Apache-2.0
  SRC controller/chip.cc
      controller/chip.h
      controller/controller.cc
      controller/controller.h
      controller/device.cc
      controller/device.h
      controller/device_notify_manager.cc
      controller/device_notify_manager.h
      controller/scene_controller.cc
      controller/scene_controller.h
  DEPS grpc++ hci-lib libbt-rootcanal netsim-cli-proto-lib util-lib wifi-lib)
target_include_directories(controller-lib PRIVATE .)

android_add_library(
  TARGET frontend-client LICENSE Apache-2.0 SRC frontend/frontend_client.cc
                                                frontend/frontend_client.h
  DEPS grpc++ netsim-cli-proto-lib protobuf::libprotobuf util-lib)
target_include_directories(frontend-client PRIVATE .)

android_add_library(
  TARGET frontend-lib
  LICENSE Apache-2.0
  SRC frontend/ frontend/frontend_server.cc frontend/frontend_server.h
      frontend/server_response_writable.h
  DEPS controller-lib grpc++ netsim-cli-proto-lib netsim-cxx-generated
       # TODO: refactor so this isn't needed
       protobuf::libprotobuf util-lib)
target_include_directories(frontend-lib PRIVATE . ../rust/netsim-cxx
                                                ../rust/netsim-cxx/cxx)

android_add_library(
  TARGET packet-streamer-server-lib
  LICENSE Apache-2.0
  SRC backend/backend_packet_hub.h backend/grpc_server.cc backend/grpc_server.h
  DEPS grpc++
       libbt-rootcanal
       netsim-cxx
       netsim-cxx-generated
       netsimd-proto-lib
       protobuf::libprotobuf
       util-lib)
target_include_directories(packet-streamer-server-lib
                           PRIVATE . ../rust/netsim-cxx ../rust/netsim-cxx/cxx)

android_add_library(
  TARGET packet-streamer-client-lib
  LICENSE Apache-2.0
  SRC backend/packet_streamer_client.cc backend/packet_streamer_client.h
  DEPS android-emu-base
       android-emu-base-headers
       android-emu-base-process
       grpc++
       packet-streamer-proto-lib
       protobuf::libprotobuf
       util-lib)
target_include_directories(packet-streamer-client-lib
                           PUBLIC ${CMAKE_CURRENT_LIST_DIR})

android_add_library(
  TARGET hci-lib
  LICENSE Apache-2.0
  SRC hci/bluetooth_facade.cc hci/bluetooth_facade.h hci/hci_debug.cc
      hci/hci_debug.h hci/hci_packet_transport.cc hci/hci_packet_transport.h
  DEPS android-emu-base
       android-emu-base-headers
       controller-lib
       grpc++
       libbt-rootcanal
       libbt-rootcanal-qemu
       netsimd-proto-lib)
target_include_directories(hci-lib PRIVATE ${PROTOBUF_INCLUDE_DIR} .
                                           ../rust/netsim-cxx/cxx)
target_compile_definitions(hci-lib PUBLIC NETSIM_ANDROID_EMULATOR)

android_add_library(
  TARGET wifi-lib
  LICENSE Apache-2.0
  SRC wifi/wifi_facade.cc wifi/wifi_facade.h wifi/wifi_packet_hub.h
  DEPS controller-lib netsim-cli-proto-lib packet-streamer-server-lib
       protobuf::libprotobuf)
target_include_directories(wifi-lib PRIVATE ${PROTOBUF_INCLUDE_DIR} .)
target_compile_definitions(wifi-lib PUBLIC NETSIM_ANDROID_EMULATOR)

android_add_library(
  TARGET packet-hub-lib LICENSE Apache-2.0 SRC packet_hub/packet_hub.cc
                                               packet_hub/packet_hub.h
  DEPS grpc++ netsim-cli-proto-lib packet-streamer-server-lib)
target_include_directories(packet-hub-lib PRIVATE ${PROTOBUF_INCLUDE_DIR} .
                                                  ../rust/netsim-cxx/cxx)

add_library(netsim-lib INTERFACE)
target_link_libraries(
  netsim-lib
  INTERFACE core-lib
            netsim-cli-proto-lib
            frontend-lib
            hci-lib
            netsimd-proto-lib
            packet-hub-lib
            packet-streamer-proto-lib
            packet-streamer-client-lib
            packet-streamer-server-lib
            protobuf::libprotobuf
            util-lib)

android_license(TARGET "netsim-lib" LIBNAME None SPDX None LICENSE None
                LOCAL None)
