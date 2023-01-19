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

// Frontend command line interface.
#pragma once

#include <memory>
#include <string_view>

#include "../rust/frontend-client-cxx/cxx/cxx.h"
#include "frontend.grpc.pb.h"

namespace netsim {
namespace frontend {

class ClientResult {
 public:
  ClientResult(bool is_ok, const std::string err, const std::string json)
      : is_ok_(is_ok), err_(err), json_(std::move(json)){};

  bool IsOk() const { return is_ok_; };
  rust::String Err() const { return err_; };
  rust::String Json() const { return json_; };

 private:
  bool is_ok_;
  std::string err_;
  std::string json_;
};

class FrontendClient {
 public:
  virtual ~FrontendClient(){};
  virtual std::unique_ptr<ClientResult> GetVersion() const = 0;
  virtual std::unique_ptr<ClientResult> GetDevices() const = 0;
};

std::unique_ptr<FrontendClient> NewFrontendClient();

}  // namespace frontend
}  // namespace netsim
