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

use super::advertise_data::{AdvertiseData, AdvertiseDataBuilder};
use super::advertise_settings::{
    AdvertiseMode, AdvertiseSettings, AdvertiseSettingsBuilder, TxPowerLevel,
};
use super::chip::{rust_bluetooth_add, RustBluetoothChipCallbacks};
use crate::devices::chip::{ChipIdentifier, FacadeIdentifier};
use crate::devices::device::{AddChipResult, DeviceIdentifier};
use crate::devices::{devices_handler::add_chip, id_factory::IdFactory};
use crate::ffi::{generate_advertising_packet, generate_scan_response_packet, RustBluetoothChip};
use cxx::{let_cxx_string, UniquePtr};
use lazy_static::lazy_static;
use log::{error, info, warn};
use netsim_proto::common::ChipKind;
use netsim_proto::model::chip::{
    bluetooth_beacon::AdvertiseData as AdvertiseDataProto,
    bluetooth_beacon::AdvertiseSettings as AdvertiseSettingsProto,
    BluetoothBeacon as BluetoothBeaconProto,
};
use netsim_proto::model::chip_create::{
    BluetoothBeaconCreate as BluetoothBeaconCreateProto, Chip as BuiltinProto,
};
use netsim_proto::model::{ChipCreate as ChipCreateProto, DeviceCreate as DeviceCreateProto};
use protobuf::MessageField;
use std::alloc::System;
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};
use std::{collections::HashMap, ptr::null};

// PhyType::LOW_ENERGY defined in packages/modules/Bluetooth/tools/rootcanal/include/phy.h
static PHY_TYPE_LE: u8 = 0;

lazy_static! {
    // A singleton that contains a hash map from chip id to RustBluetoothChip.
    // It's used by `BeaconChip` to access `RustBluetoothChip` to call send_link_layer_packet().
    static ref BT_CHIPS: RwLock<HashMap<ChipIdentifier, Mutex<UniquePtr<RustBluetoothChip>>>> =
        RwLock::new(HashMap::new());
    // Used to find beacon chip based on it's id from static methods.
    pub(crate) static ref BEACON_CHIPS: RwLock<HashMap<ChipIdentifier, Mutex<BeaconChip>>> =
        RwLock::new(HashMap::new());
}

/// BeaconChip class.
pub struct BeaconChip {
    device_name: String,
    chip_id: ChipIdentifier,
    address: String,
    advertise_settings: AdvertiseSettings,
    advertise_data: AdvertiseData,
    advertise_last: Option<Instant>,
    advertise_start: Option<Instant>,
}

impl BeaconChip {
    pub fn new(device_name: String, chip_id: ChipIdentifier, address: String) -> Self {
        BeaconChip {
            chip_id,
            device_name: device_name.clone(),
            address,
            advertise_settings: AdvertiseSettings::builder().build(),
            advertise_data: AdvertiseData::builder(device_name, TxPowerLevel::default())
                .build()
                .unwrap(),
            advertise_last: None,
            advertise_start: None,
        }
    }

    pub fn from_proto(
        device_name: String,
        chip_id: ChipIdentifier,
        beacon_proto: &BluetoothBeaconCreateProto,
    ) -> Result<Self, String> {
        let advertise_settings = AdvertiseSettings::from_proto(&beacon_proto.settings)?;
        let advertise_data = AdvertiseData::from_proto(
            device_name.clone(),
            beacon_proto
                .settings
                .tx_power
                .as_ref()
                .map(TxPowerLevel::try_from)
                .transpose()?
                .unwrap_or_default(),
            &beacon_proto.adv_data,
        )?;

        Ok(BeaconChip {
            device_name,
            chip_id,
            address: beacon_proto.address.clone(),
            advertise_settings,
            advertise_data,
            advertise_last: None,
            advertise_start: None,
        })
    }

    pub fn send_link_layer_packet(&self, packet: &[u8], packet_type: u8, tx_power: i8) {
        let binding = BT_CHIPS.read().unwrap();
        if let Some(rust_bluetooth_chip) = binding.get(&self.chip_id) {
            rust_bluetooth_chip.lock().unwrap().pin_mut().send_link_layer_packet(
                packet,
                packet_type,
                tx_power,
            );
        } else {
            warn!("Failed to get RustBluetoothChip for unknown chip id: {}", self.chip_id);
        };
    }
}

// BEACON_CHIPS has ownership of all the BeaconChips, so we need a separate class to hold the callbacks.
// This class will be owned by rootcanal.
pub struct BeaconChipCallbacks {
    chip_id: ChipIdentifier,
}

impl RustBluetoothChipCallbacks for BeaconChipCallbacks {
    fn tick(&mut self) {
        let guard = BEACON_CHIPS.read().unwrap();
        let mut beacon = guard.get(&self.chip_id);
        if beacon.is_none() {
            error!("could not find bluetooth beacon with chip id {}", self.chip_id);
            return;
        }
        let mut beacon = beacon.unwrap().lock().unwrap();

        if let (Some(start), Some(timeout)) =
            (beacon.advertise_start, beacon.advertise_settings.timeout)
        {
            if start.elapsed() > timeout {
                return;
            }
        }

        if let Some(last) = beacon.advertise_last {
            if last.elapsed() <= beacon.advertise_settings.mode.interval {
                return;
            }
        } else {
            beacon.advertise_start = Some(Instant::now())
        }

        beacon.advertise_last = Some(Instant::now());
        let packet = generate_advertising_packet(&beacon.address, beacon.advertise_data.as_bytes());
        beacon.send_link_layer_packet(
            &packet,
            PHY_TYPE_LE,
            beacon.advertise_settings.tx_power_level.dbm,
        );
    }

    fn receive_link_layer_packet(
        &mut self,
        source_address: String,
        destination_address: String,
        packet_type: u8,
        packet: &[u8],
    ) {
        let guard = BEACON_CHIPS.read().unwrap();
        let mut beacon = guard.get(&self.chip_id);
        if beacon.is_none() {
            error!("could not find bluetooth beacon with chip id {}", self.chip_id);
            return;
        }
        let mut beacon = beacon.unwrap().lock().unwrap();

        if !beacon.advertise_settings.scannable {
            return;
        }

        // TODO(jmes): Implement by following the example of Beacon::ReceiveLinkLayerPacket()
        //             in packages/modules/Bluetooth/tools/rootcanal/model/devices/beacon.cc.
        warn!("scan response is not yet implemented");
    }
}

/// Add a beacon device in rootcanal.
///
/// Called by `devices/chip.rs`.
///
/// Similar to `bluetooth_add()`.
#[cfg(not(test))]
pub fn bluetooth_beacon_add(
    device_id: DeviceIdentifier,
    device_name: String,
    chip_id: ChipIdentifier,
    chip_proto: &ChipCreateProto,
) -> Result<FacadeIdentifier, String> {
    let beacon_proto = match &chip_proto.chip {
        Some(BuiltinProto::BleBeacon(beacon_proto)) => beacon_proto,
        _ => return Err(String::from("failed to create ble beacon: unexpected chip type")),
    };

    let beacon_chip = BeaconChip::from_proto(device_name, chip_id, beacon_proto)?;
    if BEACON_CHIPS.write().unwrap().insert(chip_id, Mutex::new(beacon_chip)).is_some() {
        return Err(format!(
            "failed to create a bluetooth beacon chip with id {chip_id}: chip id already exists.",
        ));
    }

    let callbacks: Box<dyn RustBluetoothChipCallbacks> = Box::new(BeaconChipCallbacks { chip_id });
    let add_rust_device_result = rust_bluetooth_add(
        device_id,
        callbacks,
        String::from("beacon"),
        beacon_proto.address.clone(),
    );
    let rust_chip = add_rust_device_result.rust_chip;
    let facade_id = add_rust_device_result.facade_id;
    info!(
        "Creating HCI facade_id: {} for device_id: {} chip_id: {}",
        facade_id, device_id, chip_id
    );
    BT_CHIPS.write().unwrap().insert(chip_id, Mutex::new(rust_chip));

    Ok(facade_id)
}

// TODO(jmes) Support removing Beacon (b/292234625).

pub fn bluetooth_beacon_patch(
    chip_id: ChipIdentifier,
    patch: &BluetoothBeaconProto,
) -> Result<(), String> {
    let mut guard = BEACON_CHIPS.write().unwrap();
    let mut beacon = guard
        .get_mut(&chip_id)
        .ok_or(format!("could not find bluetooth beacon with chip id {chip_id} for patching"))?
        .get_mut()
        .unwrap();

    if patch.address != String::default() {
        beacon.address = patch.address.clone();
    }

    if let Some(patch_settings) = patch.settings.as_ref() {
        if let Some(interval) = patch_settings.interval.as_ref() {
            beacon.advertise_settings.mode = interval.into();
        }

        if let Some(tx_power) = patch_settings.tx_power.as_ref() {
            beacon.advertise_settings.tx_power_level = tx_power.try_into()?
        }

        beacon.advertise_settings.scannable =
            patch_settings.scannable || beacon.advertise_settings.scannable;

        if patch_settings.timeout != u64::default() {
            beacon.advertise_settings.timeout = Some(Duration::from_millis(patch_settings.timeout));
        }
    }

    if let Some(patch_adv_data) = patch.adv_data.as_ref() {
        let mut builder = AdvertiseData::builder(
            beacon.device_name.clone(),
            beacon.advertise_settings.tx_power_level,
        );

        if patch_adv_data.include_device_name || beacon.advertise_data.include_device_name {
            builder.include_device_name();
        }

        if patch_adv_data.include_tx_power_level || beacon.advertise_data.include_tx_power_level {
            builder.include_tx_power_level();
        }

        if !patch_adv_data.manufacturer_data.is_empty() {
            builder.manufacturer_data(patch_adv_data.manufacturer_data.clone());
        } else if let Some(manufacturer_data) = beacon.advertise_data.manufacturer_data.as_ref() {
            builder.manufacturer_data(manufacturer_data.clone());
        }

        beacon.advertise_data = builder.build()?;
    }

    Ok(())
}

pub fn bluetooth_beacon_get(chip_id: ChipIdentifier) -> Result<BluetoothBeaconProto, String> {
    let guard = BEACON_CHIPS.read().unwrap();
    let beacon = guard
        .get(&chip_id)
        .ok_or(format!("could not get bluetooth beacon with chip id {chip_id}"))?
        .lock()
        .unwrap();

    Ok(BluetoothBeaconProto {
        address: beacon.address.clone(),
        settings: MessageField::some((&beacon.advertise_settings).try_into()?),
        adv_data: MessageField::some((&beacon.advertise_data).into()),
        ..Default::default()
    })
}

#[cfg(test)]
pub mod tests {
    use std::thread;

    use netsim_proto::model::chip::bluetooth_beacon::{
        advertise_settings::{AdvertiseTxPower as AdvertiseTxPowerProto, Tx_power as TxPowerProto},
        AdvertiseData as AdvertiseDataProto,
    };

    use super::*;
    use crate::bluetooth::{bluetooth_beacon_add, refresh_resource};

    lazy_static! {
        static ref TEST_GUID_GENERATOR: Mutex<IdFactory<u32>> = Mutex::new(IdFactory::new(0, 1));
    }

    fn new_test_beacon_with_settings(settings: AdvertiseSettingsProto) -> DeviceIdentifier {
        let id = TEST_GUID_GENERATOR.lock().unwrap().next_id();

        let add_result = bluetooth_beacon_add(
            0,
            format!("test-device-{:?}", thread::current().id()),
            id,
            &ChipCreateProto {
                name: format!("test-beacon-chip-{:?}", thread::current().id()),
                chip: Some(BuiltinProto::BleBeacon(BluetoothBeaconCreateProto {
                    address: String::from("00:00:00:00:00:00"),
                    settings: MessageField::some(settings),
                    ..Default::default()
                })),
                ..Default::default()
            },
        );
        assert!(add_result.is_ok(), "{}", add_result.unwrap_err());

        id
    }

    fn cleanup_beacon(chip_id: ChipIdentifier) {
        BEACON_CHIPS.write().unwrap().remove(&chip_id);
    }

    #[test]
    fn test_beacon_get() {
        let interval = Duration::from_millis(9999);
        let settings = AdvertiseSettingsProto {
            interval: Some(AdvertiseMode::new(interval).try_into().unwrap()),
            ..Default::default()
        };

        let id = new_test_beacon_with_settings(settings);

        let beacon = bluetooth_beacon_get(id);
        assert!(beacon.is_ok(), "{}", beacon.unwrap_err());
        let beacon = beacon.unwrap();

        let interval_after_get =
            beacon.settings.interval.as_ref().map(AdvertiseMode::from).unwrap().interval;

        assert_eq!(interval, interval_after_get);
        cleanup_beacon(id);
    }

    #[test]
    fn test_beacon_patch() {
        let settings = AdvertiseSettingsProto {
            interval: Some(AdvertiseMode::new(Duration::from_millis(0)).try_into().unwrap()),
            ..Default::default()
        };

        let id = new_test_beacon_with_settings(settings);

        let interval = Duration::from_millis(33);
        let tx_power = TxPowerProto::TxPowerLevel(AdvertiseTxPowerProto::MEDIUM.into());
        let scannable = true;
        let patch_result = bluetooth_beacon_patch(
            id,
            &BluetoothBeaconProto {
                settings: MessageField::some(AdvertiseSettingsProto {
                    interval: Some(
                        AdvertiseMode::new(Duration::from_millis(33)).try_into().unwrap(),
                    ),
                    scannable,
                    tx_power: Some(tx_power.clone()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
        assert!(patch_result.is_ok(), "{}", patch_result.unwrap_err());

        let beacon_proto = bluetooth_beacon_get(id);
        assert!(beacon_proto.is_ok(), "{}", beacon_proto.unwrap_err());
        let beacon_proto = beacon_proto.unwrap();
        let interval_after_patch =
            beacon_proto.settings.interval.as_ref().map(AdvertiseMode::from).unwrap().interval;

        assert_eq!(interval, interval_after_patch);
        assert_eq!(tx_power, *beacon_proto.settings.tx_power.as_ref().unwrap());
        assert_eq!(scannable, beacon_proto.settings.scannable);
        cleanup_beacon(id);
    }

    #[test]
    fn test_beacon_patch_default() {
        let settings =
            AdvertiseSettingsProto { timeout: 1234, scannable: true, ..Default::default() };

        let id = new_test_beacon_with_settings(settings.clone());

        let patch_result = bluetooth_beacon_patch(id, &BluetoothBeaconProto::default());
        assert!(patch_result.is_ok(), "{}", patch_result.unwrap_err());

        let beacon_proto = bluetooth_beacon_get(id);
        assert!(beacon_proto.is_ok(), "{}", beacon_proto.unwrap_err());
        let beacon_proto = beacon_proto.unwrap();

        let settings_after_patch = beacon_proto.settings.unwrap();
        assert_eq!(settings.timeout, settings_after_patch.timeout);
        assert_eq!(settings.scannable, settings_after_patch.scannable);
    }
}