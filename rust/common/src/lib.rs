//
//  Copyright 2023 Google, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at:
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! # netsim-common Crate
//!
//! A collection of utilities for netsimd and netsim.

pub mod system;
pub mod util;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    // Shared mutex to ensure environment is not modified by multiple tests simultaneously
    pub static ENV_MUTEX: Mutex<()> = Mutex::new(());
}
