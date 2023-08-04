// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frontend_proto::model::{
    self,
    chip::bluetooth_beacon::{AdvertiseData, AdvertiseSettings},
};
use protobuf::MessageField;
use std::fmt;

pub const INDENT_INCREMENT: usize = 2;

/// Displayer for model protobufs. Implements fmt::Display.
/// # Invariants
/// Displayed values **do not** end in a newline.
pub struct Displayer<T> {
    value: T,
    verbose: bool,
    indent: usize,
}

impl<T> Displayer<T> {
    /// Returns a new displayer for values of the provided type.
    pub fn new(value: T, verbose: bool) -> Self {
        Displayer { value, verbose, indent: 0 }
    }

    /// Indent the displayed string by a given amount. Returns `self`.
    pub fn indent(&mut self, indent: usize) -> &Self {
        self.indent = indent;
        self
    }
}

impl fmt::Display for Displayer<&model::chip::BluetoothBeacon> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let address_width = 16;
        write!(f, "{:indent$}address: {:address_width$}", "", self.value.address)?;

        if self.value.settings.is_some()
            && self.value.settings != MessageField::some(AdvertiseSettings::default())
        {
            writeln!(f)?;
            write!(
                f,
                "{:indent$}advertise settings:\n{}",
                "",
                Displayer::new(self.value.settings.as_ref().unwrap_or_default(), self.verbose)
                    .indent(self.indent + INDENT_INCREMENT)
            )?;
        }

        if self.value.adv_data.is_some()
            && self.value.adv_data != MessageField::some(AdvertiseData::default())
        {
            writeln!(f)?;
            write!(
                f,
                "{:indent$}advertise packet data:\n{}",
                "",
                Displayer::new(self.value.adv_data.as_ref().unwrap_or_default(), self.verbose)
                    .indent(self.indent + INDENT_INCREMENT)
            )?;
        }

        // TODO(jmes): Add scan response data.

        Ok(())
    }
}

impl fmt::Display for Displayer<&model::Position> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let precision = 2;

        if self.verbose
            || (self.value.x != f32::default()
                || self.value.y != f32::default()
                || self.value.z != f32::default())
        {
            write!(
                f,
                "{:indent$} position: {:.precision$}, {:.precision$}, {:.precision$}",
                "", self.value.x, self.value.y, self.value.z,
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for Displayer<&model::chip::bluetooth_beacon::AdvertiseSettings> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let width = 25;
        if self.value.tx_power_level != i32::default() {
            writeln!(
                f,
                "{:indent$}{:width$}: {} dBm",
                "", "tx power level", self.value.tx_power_level
            )?;
        }

        if self.value.interval != u64::default() {
            write!(f, "{:indent$}{:width$}: {} ms", "", "interval", self.value.interval)?;
        }

        Ok(())
    }
}

impl fmt::Display for Displayer<&model::chip::bluetooth_beacon::AdvertiseData> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let width = 25;

        if self.value.include_device_name != bool::default() {
            writeln!(
                f,
                "{:indent$}{:width$}: {}",
                "", "include device name", self.value.include_device_name
            )?;
        }

        if self.value.include_tx_power_level != bool::default() {
            writeln!(
                f,
                "{:indent$}{:width$}: {}",
                "", "include tx power level", self.value.include_tx_power_level
            )?;
        }

        if self.value.manufacturer_data != Vec::<u8>::default() {
            write!(
                f,
                "{:indent$}{:width$}: {}",
                "",
                "manufacturer data bytes",
                self.value.manufacturer_data.len()
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for Displayer<&model::chip::Radio> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let count_width = 9;
        write!(
            f,
            "{:indent$}{}",
            "",
            Displayer::new(&self.value.state.enum_value_or_default(), self.verbose),
        )?;

        if self.verbose {
            write!(
                f,
                "| rx_count: {:count_width$} | tx_count: {:count_width$}",
                self.value.rx_count, self.value.tx_count
            )?
        }

        Ok(())
    }
}

impl fmt::Display for Displayer<&model::State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent;
        let width = 9;

        write!(
            f,
            "{:indent$}{:width$}",
            "",
            match self.value {
                model::State::ON => "up",
                model::State::OFF => "down",
                _ => "unknown",
            }
        )
    }
}
