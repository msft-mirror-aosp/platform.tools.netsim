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

use frontend_proto::model::chip::{
    bluetooth_beacon::advertise_settings::Advertise_mode,
    bluetooth_beacon::advertise_settings::Tx_power_level,
};
use std::time::Duration;

// Default parameter value for SendLinkLayerPacket in packages/modules/Bluetooth/tools/model/devices/device.h
static DEFAULT_TX_POWER_LEVEL: i8 = 0;
// From Beacon::Beacon constructor referenced in packages/modules/Bluetooth/tools/model/devices/beacon.cc
static DEFAULT_ADVERTISE_INTERVAL: Duration = Duration::from_millis(1280);

// TODO(jmes): Implement advertise settings builder b/294598163

/// A ble beacon advertise mode. Can be casted to/from a(n):
/// * `std::time::Duration` representing the time interval between advertisements
/// * `model::chip::bluetooth_beacon::advertise_settings::Advertise_mode`
pub struct AdvertiseMode {
    interval: Duration,
}

impl Default for AdvertiseMode {
    fn default() -> Self {
        Self { interval: DEFAULT_ADVERTISE_INTERVAL }
    }
}

impl From<&Advertise_mode> for AdvertiseMode {
    fn from(value: &Advertise_mode) -> Self {
        match value {
            Advertise_mode::ModeNumeric(interval) => {
                Self { interval: Duration::from_millis(*interval) }
            }
            // TODO(jmes): Support named advertising modes b/294260722
            _ => todo!("named advertising modes are not yet implemented"),
        }
    }
}

impl TryFrom<AdvertiseMode> for Advertise_mode {
    type Error = String;

    fn try_from(value: AdvertiseMode) -> Result<Self, Self::Error> {
        Ok(Advertise_mode::ModeNumeric(value.interval.as_millis().try_into().map_err(|_| {
            String::from(
                "failed to convert duration to AdvertiseMode: number of milliseconds was larger than a u64",
            )
        })?))
    }
}

impl From<AdvertiseMode> for Duration {
    fn from(value: AdvertiseMode) -> Self {
        value.interval
    }
}

impl From<Duration> for AdvertiseMode {
    fn from(value: Duration) -> Self {
        Self { interval: value }
    }
}

/// A ble beacon transmit power level. Can be casted to/from a(n):
/// * `i8` measuring power in dBm
/// * `model::chip::bluetooth_beacon::advertise_settings::Tx_power_level`
pub struct TxPowerLevel {
    tx_power: i8,
}

impl Default for TxPowerLevel {
    fn default() -> Self {
        TxPowerLevel { tx_power: DEFAULT_TX_POWER_LEVEL }
    }
}

impl TryFrom<&Tx_power_level> for TxPowerLevel {
    type Error = String;

    fn try_from(value: &Tx_power_level) -> Result<Self, Self::Error> {
        Ok(match value {
            Tx_power_level::LevelNumeric(tx_power) => Self {
                tx_power: (*tx_power)
                    .try_into()
                    .map_err(|_| "tx power level was too large: it must fit in an i8")?,
            },
            // TODO(jmes): Support named tx power levels b/294260722
            _ => todo!("named tx power levels are not yet implemented"),
        })
    }
}

impl From<TxPowerLevel> for Tx_power_level {
    fn from(value: TxPowerLevel) -> Self {
        Tx_power_level::LevelNumeric(value.tx_power.into())
    }
}

impl From<TxPowerLevel> for i8 {
    fn from(value: TxPowerLevel) -> Self {
        value.tx_power
    }
}

impl From<i8> for TxPowerLevel {
    fn from(value: i8) -> Self {
        Self { tx_power: value }
    }
}
