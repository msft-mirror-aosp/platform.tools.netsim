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

#if defined(_WIN32)
#include <msvc-getopt.h>
#else
#include <getopt.h>
#endif

#if defined(__linux__)
#include <execinfo.h>
#include <signal.h>
#include <unistd.h>

#include <cstdio>
#include <cstdlib>
#endif

#include "core/server.h"
#include "frontend/frontend_client_stub.h"
#include "netsim-cxx/src/lib.rs.h"
#include "util/os_utils.h"

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

constexpr int DEFAULT_HCI_PORT = 6402;

int get_hci_port(int hci_port_flag) {
  // The following priorities are used to determine the HCI port number:
  //
  // 1. The CLI flag `-hci_port`.
  // 2. The environment variable `NETSIM_HCI_PORT`.
  // 3. The default value `DEFAULT_HCI_PORT`.
  int hci_port = 0;
  if (hci_port_flag != 0) {
    hci_port = hci_port_flag;
  } else if (auto netsim_hci_port =
                 netsim::osutils::GetEnv("NETSIM_HCI_PORT", "0");
             netsim_hci_port != "0") {
    char *ptr;
    hci_port = strtol(netsim_hci_port.c_str(), &ptr, 10);
  } else {
    hci_port = DEFAULT_HCI_PORT;
  }
  return hci_port;
}

void ArgError(char *argv[], int c) {
  std::cerr << argv[0] << ": invalid option -- " << (char)c << "\n";
  std::cerr << "Try `" << argv[0] << " --help' for more information.\n";
}

int main(int argc, char *argv[]) {
#if defined(__linux__)
  signal(SIGSEGV, SignalHandler);
#endif
  bool no_web_ui = false;
  bool no_cli_ui = false;

  const char *kShortOpt = "s:dg";
  const option kLongOptions[] = {
      {"no_cli_ui", no_argument, 0, 'f'},
      {"no_web_ui", no_argument, 0, 'w'},
      {"rootcanal_controller_properties_file", required_argument, 0, 'p'},
      {"hci_port", required_argument, 0, 'b'},
  };

  bool dev = false;
  std::string fd_startup_str;
  std::string rootcanal_controller_properties_file;
  int hci_port_flag = 0;

  int c;

  while ((c = getopt_long(argc, argv, kShortOpt, kLongOptions, nullptr)) !=
         -1) {
    switch (c) {
#ifdef NETSIM_ANDROID_EMULATOR
      case 'g':
      //TODO: Remove the no-op flag after a release cycle.
        break;
#else
      case 's':
        fd_startup_str = std::string(optarg);
        break;
#endif
      case 'd':
        dev = true;
        break;

      case 'p':
        rootcanal_controller_properties_file = std::string(optarg);
        break;

      case 'f':
        no_cli_ui = true;
        break;

      case 'w':
        no_web_ui = true;
        break;

      case 'b':
        hci_port_flag = std::atoi(optarg);
        break;

      default:
        ArgError(argv, c);
        return (-2);
    }
  }

  int hci_port = get_hci_port(hci_port_flag);
  // Daemon mode -- start radio managers
  // get netsim daemon, starting if it doesn't exist
  // Create a frontend grpc client to check if a netsimd is already running.
  auto frontend_stub = netsim::frontend::NewFrontendClient();
  if (frontend_stub != nullptr) {
    std::cerr << "Failed to start netsim daemon because a netsim daemon is "
                 "already running\n";
    return -1;
  }

#ifndef NETSIM_ANDROID_EMULATOR
  if (fd_startup_str.empty()) {
    std::cerr << "Failed to start netsim daemon because fd startup flag `-s` "
                 "is empty\n";
    return -1;
  }
  netsim::RunFdTransport(fd_startup_str);
#endif

  netsim::server::Run({.dev = dev,
                       .no_cli_ui = no_cli_ui,
                       .no_web_ui = no_web_ui,
                       .hci_port = hci_port});
  return -1;
}
