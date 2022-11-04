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

#include <getopt.h>
#if defined(__linux__)
#include <execinfo.h>
#include <signal.h>
#include <unistd.h>

#include <cstdio>
#endif

#include "core/server.h"
#ifdef NETSIM_ANDROID_EMULATOR
#include "core/server_rpc.h"
#endif
#include "frontend/cli.h"
#include "hci/bluetooth_facade.h"

// Wireless network simulator for android (and other) emulated devices.

#if defined(__linux__)
// Signal handler to print backtraces and then terminate the program.
void SignalHandler(int sig) {
  size_t buffer_size = 20;  // Number of entries in that array.
  void *buffer[buffer_size];

  auto size = backtrace(buffer, buffer_size);
  fprintf(stderr,
          "netsim error: interrupt by signal %d. Obtained %d stack frames:\n",
          sig, size);
  backtrace_symbols_fd(buffer, size, STDERR_FILENO);
  exit(sig);
}
#endif

void ArgError(char *argv[], int c) {
  std::cerr << argv[0] << ": invalid option -- " << (char)c << "\n";
  std::cerr << "Try `" << argv[0] << " --help' for more information.\n";
}

int main(int argc, char *argv[]) {
#if defined(__linux__)
  signal(SIGSEGV, SignalHandler);
#endif
  const char *kShortOpt = "s:dg";
  const option kLongOptions[] = {
      {"rootcanal_default_commands_file", required_argument, 0, 'c'},
      {"rootcanal_controller_properties_file", required_argument, 0, 'p'},
  };

  bool debug = false;
  bool grpc_startup = false;
  std::string fd_startup_str;
  std::string rootcanal_default_commands_file;
  std::string rootcanal_controller_properties_file;

  int c;

  while ((c = getopt_long(argc, argv, kShortOpt, kLongOptions, nullptr)) !=
         -1) {
    switch (c) {
      case 's':
        fd_startup_str = std::string(optarg);
        break;
#ifdef NETSIM_ANDROID_EMULATOR
      case 'g':
        grpc_startup = true;
        break;
#endif
      case 'd':
        debug = true;
        break;

      case 'c':
        rootcanal_default_commands_file = std::string(optarg);
        break;

      case 'p':
        rootcanal_controller_properties_file = std::string(optarg);
        break;

      default:
        ArgError(argv, c);
        return (-2);
    }
  }

  // Daemon mode -- start radio managers
  if (!fd_startup_str.empty() || grpc_startup) {
    netsim::hci::BluetoothChipEmulator::Get().Start(
        rootcanal_default_commands_file, rootcanal_controller_properties_file);
  }

  if (!fd_startup_str.empty()) {
    netsim::StartWithFds(fd_startup_str, debug);
    return -1;
  }

  // get netsim daemon, starting if it doesn't exist
  auto frontend_stub = netsim::NewFrontendStub();
#ifdef NETSIM_ANDROID_EMULATOR
  if (frontend_stub == nullptr) {
    // starts netsim in vhci connection mode
    frontend_stub = netsim::StartWithGrpc(debug);
  }
#endif
  // could not start the server
  if (frontend_stub == nullptr) return (1);

  if (!grpc_startup) {
    std::vector<std::string_view> args(argv + 1, argv + argc);
    netsim::SendCommand(std::move(frontend_stub), args);
  }

  return (0);
}