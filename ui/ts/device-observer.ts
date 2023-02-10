// URL for netsim
const GET_DEVICES_URL = 'http://localhost:7681/get-devices';
const PATCH_DEVICE_URL = 'http://localhost:7681/patch-device';

/**
 * Interface for a method in notifying the subscribed observers.
 * Subscribed observers must implement this interface.
 */
export interface Notifiable {
  onNotify(data: {}): void;
}

// TODO(b/255353541): import message interfaces in model.proto
interface Radio {
  state?: string;
  range?: number;
  txCount?: number;
  rxCount?: number;
}

interface Bluetooth {
  lowEnergy?: Radio;
  classic?: Radio;
}

interface Chip {
  chipId?: string;
  manufacturer?: string;
  model?: string;
  capture?: string;
  bt?: Bluetooth;
  uwb?: Radio;
  wifi?: Radio;
}

/**
 * Data structure of Device from protobuf.
 * Used as a reference for subscribed observers to get proper attributes.
 * TODO: use ts-proto to auto translate protobuf messaegs to TS interfaces
 */
export interface ProtoDevice {
  deviceSerial: string;
  name?: string;
  position?: {
    x?: number;
    y?: number;
    z?: number;
  };
  orientation?: {
    yaw?: number;
    pitch?: number;
    roll?: number;
  };
  chips?: Chip[];
  visible?: boolean;
}

/**
 * Modularization of Device.
 * Contains getters and setters for properties in Device interface.
 */
export class Device {
  device: ProtoDevice;

  constructor(device: ProtoDevice) {
    this.device = device;
  }

  get deviceSerial() {
    return this.device.deviceSerial;
  }

  get name() : string {
    return this.device.name ?? "";
  }

  set name(value: string) {
    this.device.name = value;
  }

  get position() : {x: number; y: number; z: number} {
    const result = {x: 0, y: 0, z: 0};
    if ("position" in this.device && this.device.position && typeof this.device.position === 'object') {
      if ("x" in this.device.position && typeof this.device.position.x === 'number') {
        result.x = this.device.position.x;
      }
      if ("y" in this.device.position && typeof this.device.position.y === 'number') {
        result.y = this.device.position.y;
      }
      if ("z" in this.device.position && typeof this.device.position.z === 'number') {
        result.z = this.device.position.z;
      }
    }
    return result;
  }

  set position(pos: {x?: number; y?: number; z?: number}) {
    this.device.position = pos;
  }

  get orientation() : {yaw: number; pitch: number; roll: number} {
    const result = {yaw: 0, pitch: 0, roll: 0};
    if ("orientation" in this.device && this.device.orientation && typeof this.device.orientation === 'object') {
      if ("yaw" in this.device.orientation && typeof this.device.orientation.yaw === 'number') {
        result.yaw = this.device.orientation.yaw;
      }
      if ("pitch" in this.device.orientation && typeof this.device.orientation.pitch === 'number') {
        result.pitch = this.device.orientation.pitch;
      }
      if ("roll" in this.device.orientation && typeof this.device.orientation.roll === 'number') {
        result.roll = this.device.orientation.roll;
      }
    }
    return result;
  }

  set orientation(ori: {yaw?: number; pitch?: number; roll?: number}) {
    this.device.orientation = ori;
  }

  // TODO modularize getters and setters for Chip Interface
  get chips() : Chip[] {
    return this.device.chips ?? [];
  }

  // TODO modularize getters and setters for Chip Interface
  set chips(value: Chip[]) {
    this.device.chips = value;
  }

  get visible() : boolean {
    return this.device.visible ?? true;
  }

  set visible(value: boolean) {
    this.device.visible = value;
  }

  toggleChipState(chip: Chip, btType?: string) {
    if ("bt" in chip && chip.bt) {
      if (typeof(btType) === 'undefined') {
        // eslint-disable-next-line
        console.log("netsim-ui: must specify lowEnergy or classic for Bluetooth");
        return;
      }
      if (btType === "lowEnergy" && "lowEnergy" in chip.bt && chip.bt.lowEnergy) {
        if ("state" in chip.bt.lowEnergy) {
          chip.bt.lowEnergy.state = chip.bt.lowEnergy.state === 'ON' ? 'OFF' : 'ON';
        }
      }
      if (btType === "classic" && "classic" in chip.bt && chip.bt.classic) {
        if ("state" in chip.bt.classic) {
          chip.bt.classic.state = chip.bt.classic.state === 'ON' ? 'OFF' : 'ON';
        }
      }
    }
    if ("wifi" in chip && chip.wifi) {
      if ("state" in chip.wifi) {
        chip.wifi.state = chip.wifi.state === 'ON' ? 'OFF' : 'ON';
      }
    }
    if ("uwb" in chip && chip.uwb) {
      if ("state" in chip.uwb) {
        chip.uwb.state = chip.uwb.state === 'ON' ? 'OFF' : 'ON';
      }
    }

  }

  toggleCapture(device: Device, chip: Chip) {
    if ("capture" in chip && chip.capture) {
      chip.capture = chip.capture === 'ON' ? 'OFF' : 'ON';
      simulationState.patchDevice({device: {
        deviceSerial: device.deviceSerial,
        chips: device.chips,
      }});
    }
  }
}

/**
 * The most recent state of the simulation.
 * Subscribed observers must refer to this info and patch accordingly.
 */
export interface SimulationInfo {
  devices: Device[];
  selectedSerial: string;
  dimension: {
    x: number;
    y: number;
    z: number;
  };
}

interface Observable {
  registerObserver(elem: Notifiable): void;
  removeObserver(elem: Notifiable): void;
}

class SimulationState implements Observable {
  private observers: Notifiable[] = [];

  private simulationInfo: SimulationInfo = {
    devices: [],
    selectedSerial: '',
    dimension: { x: 10, y: 10, z: 0 },
  };

  constructor() {
    // initial GET
    this.invokeGetDevice();
  }

  invokeGetDevice() {
    fetch(GET_DEVICES_URL)
      .then(response => response.json())
      .then(data => {
        this.fetchDevice(data.devices);
      })
      .catch(error => {
        // eslint-disable-next-line
        console.log('Cannot connect to netsim web server', error);
      });
  }

  fetchDevice(devices: ProtoDevice[]) {
    this.simulationInfo.devices = [];
    for (const device of devices) {
      this.simulationInfo.devices.push(new Device(device));
    }
    this.notifyObservers();
  }

  patchSelected(serial: string) {
    this.simulationInfo.selectedSerial = serial;
    this.notifyObservers();
  }

  handleDrop(serial: string, x: number, y: number) {
    for (const device of this.simulationInfo.devices) {
      if (serial === device.deviceSerial) {
        device.position = {x, y, z: device.position.z};
        this.patchDevice({
          device: {
            deviceSerial: serial,
            position: device.position,
          },
        });
        break;
      }
    }
  }

  patchDevice(obj: object) {
    const jsonBody = JSON.stringify(obj);
    fetch(PATCH_DEVICE_URL, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Content-Length': jsonBody.length.toString(),
      },
      body: jsonBody,
    })
      .then(response => response.json())
      .catch(error => {
        // eslint-disable-next-line
        console.error('Error:', error);
      });
    this.notifyObservers();
  }

  registerObserver(elem: Notifiable) {
    this.observers.push(elem);
    elem.onNotify(this.simulationInfo);
  }

  removeObserver(elem: Notifiable) {
    const index = this.observers.indexOf(elem);
    this.observers.splice(index, 1);
  }

  notifyObservers() {
    for (const observer of this.observers) {
      observer.onNotify(this.simulationInfo);
    }
  }

  getDeviceList() {
    return this.simulationInfo.devices;
  }
}

/** Subscribed observers must register itself to the simulationState */
export const simulationState = new SimulationState();

async function subscribe() {
  const delay = (ms: number) => new Promise(res => setTimeout(res, ms));
  while (true) {
    simulationState.invokeGetDevice();
    await delay(1000);
  }
}

subscribe();
