// Copyright 2022 Google LLC
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

use crate::args::{self, Beacon, Command};
use frontend_client_cxx::ffi::GrpcMethod;

impl args::Command {
    /// Return the respective GrpcMethod for the command
    pub fn grpc_method(&self) -> GrpcMethod {
        match self {
            Command::Version => GrpcMethod::GetVersion,
            Command::Radio(_) => GrpcMethod::PatchDevice,
            Command::Move(_) => GrpcMethod::PatchDevice,
            Command::Devices(_) => GrpcMethod::ListDevice,
            Command::Reset => GrpcMethod::Reset,
            Command::Capture(cmd) => match cmd {
                args::Capture::List(_) => GrpcMethod::ListCapture,
                args::Capture::Get(_) => GrpcMethod::GetCapture,
                args::Capture::Patch(_) => GrpcMethod::PatchCapture,
            },
            Command::Gui => {
                panic!("No GrpcMethod for Ui Command.");
            }
            Command::Artifact => {
                panic!("No GrpcMethod for Artifact Command.");
            }
            Command::Beacon(action) => match action {
                Beacon::Create(_) => GrpcMethod::CreateDevice,
                Beacon::Patch(_) => GrpcMethod::PatchDevice,
                Beacon::Remove(_) => GrpcMethod::DeleteChip,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use args::{BinaryProtobuf, NetsimArgs};
    use clap::Parser;
    use netsim_proto::frontend::{CreateDeviceRequest, PatchDeviceRequest};
    use netsim_proto::model::chip::bluetooth_beacon::AdvertiseData as AdvertiseDataProto;
    use netsim_proto::model::chip::{
        bluetooth_beacon::{
            advertise_settings::{
                AdvertiseMode as AdvertiseModeProto, AdvertiseTxPower as AdvertiseTxPowerProto,
                Interval as IntervalProto, Tx_power as TxPowerProto,
            },
            AdvertiseSettings as AdvertiseSettingsProto,
        },
        BluetoothBeacon as BluetoothBeaconProto, Chip as ChipKindProto,
    };
    use netsim_proto::model::chip_create::{
        BluetoothBeaconCreate as BluetoothBeaconCreateProto, Chip as ChipKindCreateProto,
    };
    use netsim_proto::model::{
        Chip as ChipProto, ChipCreate as ChipCreateProto, DeviceCreate as DeviceCreateProto,
    };
    use netsim_proto::{
        common::ChipKind,
        frontend,
        model::{
            self,
            chip::{Bluetooth as Chip_Bluetooth, Radio as Chip_Radio},
            Device, Position, State,
        },
    };
    use protobuf::Message;
    use protobuf::MessageField;

    fn test_command(
        command: &str,
        expected_grpc_method: GrpcMethod,
        expected_request_byte_str: BinaryProtobuf,
    ) {
        let command = NetsimArgs::parse_from(command.split_whitespace()).command;
        assert_eq!(expected_grpc_method, command.grpc_method());
        let request = command.get_request_bytes();
        assert_eq!(request, expected_request_byte_str);
    }

    #[test]
    fn test_version_request() {
        test_command("netsim-cli version", GrpcMethod::GetVersion, Vec::new())
    }

    fn get_expected_radio(name: &str, radio_type: &str, state: &str) -> BinaryProtobuf {
        let mut chip = model::Chip { ..Default::default() };
        let chip_state = match state {
            "up" => State::ON,
            _ => State::OFF,
        };
        if radio_type == "wifi" {
            let mut wifi_chip = Chip_Radio::new();
            wifi_chip.state = chip_state.into();
            chip.set_wifi(wifi_chip);
            chip.kind = ChipKind::WIFI.into();
        } else if radio_type == "uwb" {
            let mut uwb_chip = Chip_Radio::new();
            uwb_chip.state = chip_state.into();
            chip.set_uwb(uwb_chip);
            chip.kind = ChipKind::UWB.into();
        } else {
            let mut bt_chip = Chip_Bluetooth::new();
            let mut bt_chip_radio = Chip_Radio::new();
            bt_chip_radio.state = chip_state.into();
            if radio_type == "ble" {
                bt_chip.low_energy = Some(bt_chip_radio).into();
            } else {
                bt_chip.classic = Some(bt_chip_radio).into();
            }
            chip.kind = ChipKind::BLUETOOTH.into();
            chip.set_bt(bt_chip);
        }
        let mut result = frontend::PatchDeviceRequest::new();
        let mut device = Device::new();
        device.name = name.to_owned();
        device.chips.push(chip);
        result.device = Some(device).into();
        result.write_to_bytes().unwrap()
    }

    #[test]
    fn test_radio_ble() {
        test_command(
            "netsim-cli radio ble down 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "down"),
        );
        test_command(
            "netsim-cli radio ble up 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "up"),
        );
    }

    #[test]
    fn test_radio_ble_aliases() {
        test_command(
            "netsim-cli radio ble Down 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "down"),
        );
        test_command(
            "netsim-cli radio ble Up 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "up"),
        );
        test_command(
            "netsim-cli radio ble DOWN 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "down"),
        );
        test_command(
            "netsim-cli radio ble UP 1000",
            GrpcMethod::PatchDevice,
            get_expected_radio("1000", "ble", "up"),
        );
    }

    #[test]
    fn test_radio_classic() {
        test_command(
            "netsim-cli radio classic down 100",
            GrpcMethod::PatchDevice,
            get_expected_radio("100", "classic", "down"),
        );
        test_command(
            "netsim-cli radio classic up 100",
            GrpcMethod::PatchDevice,
            get_expected_radio("100", "classic", "up"),
        );
    }

    #[test]
    fn test_radio_wifi() {
        test_command(
            "netsim-cli radio wifi down a",
            GrpcMethod::PatchDevice,
            get_expected_radio("a", "wifi", "down"),
        );
        test_command(
            "netsim-cli radio wifi up b",
            GrpcMethod::PatchDevice,
            get_expected_radio("b", "wifi", "up"),
        );
    }

    #[test]
    fn test_radio_uwb() {
        test_command(
            "netsim-cli radio uwb down a",
            GrpcMethod::PatchDevice,
            get_expected_radio("a", "uwb", "down"),
        );
        test_command(
            "netsim-cli radio uwb up b",
            GrpcMethod::PatchDevice,
            get_expected_radio("b", "uwb", "up"),
        );
    }

    fn get_expected_move(name: &str, x: f32, y: f32, z: Option<f32>) -> BinaryProtobuf {
        let mut result = frontend::PatchDeviceRequest::new();
        let mut device = Device::new();
        let position = Position { x, y, z: z.unwrap_or_default(), ..Default::default() };
        device.name = name.to_owned();
        device.position = Some(position).into();
        result.device = Some(device).into();
        result.write_to_bytes().unwrap()
    }

    #[test]
    fn test_move_int() {
        test_command(
            "netsim-cli move 1 1 2 3",
            GrpcMethod::PatchDevice,
            get_expected_move("1", 1.0, 2.0, Some(3.0)),
        )
    }

    #[test]
    fn test_move_float() {
        test_command(
            "netsim-cli move 1000 1.2 3.4 5.6",
            GrpcMethod::PatchDevice,
            get_expected_move("1000", 1.2, 3.4, Some(5.6)),
        )
    }

    #[test]
    fn test_move_mixed() {
        test_command(
            "netsim-cli move 1000 1.1 2 3.4",
            GrpcMethod::PatchDevice,
            get_expected_move("1000", 1.1, 2.0, Some(3.4)),
        )
    }

    #[test]
    fn test_move_no_z() {
        test_command(
            "netsim-cli move 1000 1.2 3.4",
            GrpcMethod::PatchDevice,
            get_expected_move("1000", 1.2, 3.4, None),
        )
    }

    #[test]
    fn test_devices() {
        test_command("netsim-cli devices", GrpcMethod::ListDevice, Vec::new())
    }

    #[test]
    fn test_reset() {
        test_command("netsim-cli reset", GrpcMethod::Reset, Vec::new())
    }

    #[test]
    fn test_capture_list() {
        test_command("netsim-cli capture list", GrpcMethod::ListCapture, Vec::new())
    }

    #[test]
    fn test_capture_list_alias() {
        test_command("netsim-cli pcap list", GrpcMethod::ListCapture, Vec::new())
    }

    //TODO: Add capture patch and get tests once able to run tests with cxx definitions

    fn get_create_device_req_bytes(
        device_name: &str,
        chip_name: &str,
        settings: AdvertiseSettingsProto,
        adv_data: AdvertiseDataProto,
    ) -> Vec<u8> {
        let device = MessageField::some(DeviceCreateProto {
            name: String::from(device_name),
            chips: vec![ChipCreateProto {
                name: String::from(chip_name),
                kind: ChipKind::BLUETOOTH_BEACON.into(),
                chip: Some(ChipKindCreateProto::BleBeacon(BluetoothBeaconCreateProto {
                    settings: MessageField::some(settings),
                    adv_data: MessageField::some(adv_data),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        });

        CreateDeviceRequest { device, ..Default::default() }.write_to_bytes().unwrap()
    }

    fn get_patch_device_req_bytes(
        device_name: &str,
        chip_name: &str,
        settings: AdvertiseSettingsProto,
        adv_data: AdvertiseDataProto,
    ) -> Vec<u8> {
        let device = MessageField::some(Device {
            name: String::from(device_name),
            chips: vec![ChipProto {
                name: String::from(chip_name),
                kind: ChipKind::BLUETOOTH_BEACON.into(),
                chip: Some(ChipKindProto::BleBeacon(BluetoothBeaconProto {
                    bt: MessageField::some(Chip_Bluetooth::new()),
                    settings: MessageField::some(settings),
                    adv_data: MessageField::some(adv_data),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        });

        PatchDeviceRequest { device, ..Default::default() }.write_to_bytes().unwrap()
    }

    #[test]
    fn test_beacon_create_all_params_ble() {
        let device_name = String::from("device");
        let chip_name = String::from("chip");

        let timeout = 1234;
        let manufacturer_data = String::from("google");

        let settings = AdvertiseSettingsProto {
            interval: Some(IntervalProto::AdvertiseMode(AdvertiseModeProto::BALANCED.into())),
            tx_power: Some(TxPowerProto::TxPowerLevel(AdvertiseTxPowerProto::ULTRA_LOW.into())),
            scannable: true,
            timeout,
            ..Default::default()
        };

        let adv_data = AdvertiseDataProto {
            include_device_name: true,
            include_tx_power_level: true,
            manufacturer_data: manufacturer_data.as_bytes().to_vec(),
            ..Default::default()
        };

        let request = get_create_device_req_bytes(&device_name, &chip_name, settings, adv_data);

        test_command(
            format!(
                "netsim-cli beacon create ble {} {} --advertise-mode balanced --tx-power-level ultra-low --scannable --timeout {} --include-device-name --include-tx-power-level --manufacturer-data {}",
                device_name, chip_name, timeout, manufacturer_data
            )
            .as_str(),
            GrpcMethod::CreateDevice,
            request,
        )
    }

    #[test]
    fn test_beacon_patch_all_params_ble() {
        let device_name = String::from("device");
        let chip_name = String::from("chip");

        let interval = 1234;
        let timeout = 9999;
        let tx_power_level = -3;
        let manufacturer_data = String::from("test12345");

        let settings = AdvertiseSettingsProto {
            interval: Some(IntervalProto::Milliseconds(interval)),
            tx_power: Some(TxPowerProto::Dbm(tx_power_level)),
            scannable: true,
            timeout,
            ..Default::default()
        };
        let adv_data = AdvertiseDataProto {
            include_device_name: true,
            include_tx_power_level: true,
            manufacturer_data: manufacturer_data.as_bytes().to_vec(),
            ..Default::default()
        };

        let request = get_patch_device_req_bytes(&device_name, &chip_name, settings, adv_data);

        test_command(
            format!(
                "netsim-cli beacon patch ble {} {} --advertise-mode {} --scannable --timeout {} --tx-power-level {} --manufacturer-data {} --include-device-name --include-tx-power-level",
                device_name, chip_name, interval, timeout, tx_power_level, manufacturer_data
            )
            .as_str(),
            GrpcMethod::PatchDevice,
            request,
        )
    }

    #[test]
    fn test_beacon_create_ble_tx_power() {
        let device_name = String::from("device");
        let chip_name = String::from("chip");

        let settings = AdvertiseSettingsProto {
            tx_power: Some(TxPowerProto::TxPowerLevel(AdvertiseTxPowerProto::HIGH.into())),
            ..Default::default()
        };
        let adv_data = AdvertiseDataProto { include_tx_power_level: true, ..Default::default() };

        let request = get_create_device_req_bytes(&device_name, &chip_name, settings, adv_data);

        test_command(
            format!(
                "netsim-cli beacon create ble {} {} --tx-power-level high --include-tx-power-level",
                device_name, chip_name
            )
            .as_str(),
            GrpcMethod::CreateDevice,
            request,
        )
    }

    #[test]
    fn test_beacon_create_default() {
        let request = get_create_device_req_bytes(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        test_command("netsim-cli beacon create ble", GrpcMethod::CreateDevice, request)
    }

    #[test]
    fn test_beacon_patch_interval() {
        let device_name = String::from("device");
        let chip_name = String::from("chip");

        let settings = AdvertiseSettingsProto {
            interval: Some(IntervalProto::AdvertiseMode(AdvertiseModeProto::LOW_LATENCY.into())),
            ..Default::default()
        };

        let request =
            get_patch_device_req_bytes(&device_name, &chip_name, settings, Default::default());

        test_command(
            format!(
                "netsim-cli beacon patch ble {} {} --advertise-mode low-latency",
                device_name, chip_name
            )
            .as_str(),
            GrpcMethod::PatchDevice,
            request,
        )
    }

    #[test]
    fn test_create_beacon_negative_timeout_fails() {
        let command = String::from("netsim-cli beacon create ble --timeout -1234");
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }

    #[test]
    fn test_create_beacon_large_tx_power_fails() {
        let command =
            format!("netsim-cli beacon create ble --tx-power-level {}", (i8::MAX as i32) + 1);
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }

    #[test]
    fn test_create_beacon_unknown_mode() {
        let command = String::from("netsim-cli beacon create ble --advertise-mode not-a-mode");
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }

    #[test]
    fn test_patch_beacon_negative_timeout_fails() {
        let command = String::from("netsim-cli beacon patch ble --timeout -1234");
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }

    #[test]
    fn test_patch_beacon_large_tx_power_fails() {
        let command =
            format!("netsim-cli beacon patch ble --tx-power-level {}", (i8::MAX as i32) + 1);
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }

    #[test]
    fn test_patch_beacon_unknown_mode() {
        let command = String::from("netsim-cli beacon patch ble --advertise-mode not-a-mode");
        assert!(NetsimArgs::try_parse_from(command.split_whitespace()).is_err());
    }
}
