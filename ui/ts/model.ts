/* eslint-disable */
import type { ChipKind } from "./common";

export const protobufPackage = "netsim.model";

export enum PhyKind {
  NONE = "NONE",
  BLUETOOTH_CLASSIC = "BLUETOOTH_CLASSIC",
  BLUETOOTH_LOW_ENERGY = "BLUETOOTH_LOW_ENERGY",
  WIFI = "WIFI",
  UWB = "UWB",
  WIFI_RTT = "WIFI_RTT",
  UNRECOGNIZED = "UNRECOGNIZED",
}

/** An explicit valued boolean. */
export enum State {
  UNKNOWN = "UNKNOWN",
  ON = "ON",
  OFF = "OFF",
  UNRECOGNIZED = "UNRECOGNIZED",
}

/**
 * A 3D position. A valid Position must have both x and y coordinates.
 * The position coordinates are in meters.
 */
export interface Position {
  x: number;
  y: number;
  z: number;
}

export interface Orientation {
  yaw: number;
  pitch: number;
  roll: number;
}

export interface Chip {
  chipKind: ChipKind;
  /** optional like "rear-right" */
  chipId: string;
  /** optional like Quorvo */
  manufacturer: string;
  /** optional like DW300 */
  model: string;
  /** packet capture */
  capture: State;
  bt?: Chip_Bluetooth | undefined;
  uwb?: Chip_Radio | undefined;
  wifi?: Chip_Radio | undefined;
}

/** Radio state associated with the Chip */
export interface Chip_Radio {
  state: State;
  range: number;
  txCount: number;
  rxCount: number;
}

/** Bluetooth has 2 radios */
export interface Chip_Bluetooth {
  lowEnergy: Chip_Radio | undefined;
  classic: Chip_Radio | undefined;
}

export interface Device {
  /** settable at creation */
  name: string;
  visible: boolean;
  position: Position | undefined;
  orientation:
    | Orientation
    | undefined;
  /** Device can have multiple chips of the same kind. */
  chips: Chip[];
}

export interface Scene {
  devices: Device[];
}