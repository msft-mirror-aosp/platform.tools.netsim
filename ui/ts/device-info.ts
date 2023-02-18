import { css, html, LitElement } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { live } from 'lit/directives/live.js';
import { styleMap } from 'lit/directives/style-map.js';
import {
  Device,
  Notifiable,
  SimulationInfo,
  simulationState,
} from './device-observer.js';
import {State} from './model.js'

@customElement('ns-device-info')
export class DeviceInformation extends LitElement implements Notifiable {
  /**
   * The currently user-clicked device will have its
   * information displayed on the info panel.
   */
  @property() selectedDevice?: Device;

  /**
   * the yaw value in orientation for ns-cube-sprite
   * unit: deg
   */
  @property({ type: Number })
  yaw = 0;

  /**
   * the pitch value in orientation for ns-cube-sprite
   * unit: deg
   */
  @property({ type: Number })
  pitch = 0;

  /**
   * the roll value in orientation for ns-cube-sprite
   * unit: deg
   */
  @property({ type: Number })
  roll = 0;

  /**
   * The state of device info. True if edit mode.
   */
  @property({ type: Boolean })
  editMode = false;

  /**
   * the x value in position for ns-cube-sprite
   * unit: cm
   */
  @property({ type: Number })
  posX = 0;

  /**
   * the y value in position for ns-cube-sprite
   * unit: cm
   */
  @property({ type: Number })
  posY = 0;

  /**
   * the z value in position for ns-cube-sprite
   * unit: cm
   */
  @property({ type: Number })
  posZ = 0;

  holdRange = false;

  static styles = css`
    :host {
      cursor: pointer;
      display: grid;
      place-content: center;
      color: white;
      font-size: 25px;
      font-family: 'Lato', sans-serif;
      border: 5px solid black;
      border-radius: 12px;
      padding: 10px;
      background-color: #9199a5;
      max-width: 600px;
    }

    .title {
      font-weight: bold;
      text-transform: uppercase;
      text-align: center;
      margin-bottom: 10px;
    }

    .setting {
      display: grid;
      grid-template-columns: auto auto;
      margin-top: 0px;
      margin-bottom: 30px;
      //border: 3px solid black;
      padding: 10px;
    }

    .setting .name {
      grid-column: 1 / span 2;
      text-transform: uppercase;
      text-align: left;
      margin-bottom: 10px;
      font-weight: bold;
    }

    .label {
      grid-column: 1;
      text-align: left;
    }

    .info {
      grid-column: 2;
      text-align: right;
      margin-bottom: 10px;
    }

    .switch {
      position: relative;
      float: right;
      width: 60px;
      height: 34px;
    }

    .switch input {
      opacity: 0;
      width: 0;
      height: 0;
    }

    .slider {
      position: absolute;
      cursor: pointer;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: #ccc;
      -webkit-transition: 0.4s;
      transition: 0.4s;
    }

    .slider:before {
      position: absolute;
      content: '';
      height: 26px;
      width: 26px;
      left: 4px;
      bottom: 4px;
      background-color: white;
      -webkit-transition: 0.4s;
      transition: 0.4s;
    }

    input:checked + .slider {
      background-color: #2196f3;
    }

    input:focus + .slider {
      box-shadow: 0 0 1px #2196f3;
    }

    input:checked + .slider:before {
      -webkit-transform: translateX(26px);
      -ms-transform: translateX(26px);
      transform: translateX(26px);
    }

    /* Rounded sliders */
    .slider.round {
      border-radius: 34px;
    }

    .slider.round:before {
      border-radius: 50%;
    }

    .text {
      display: inline-block;
      position: relative;
      width: 50px;
    }

    input[type='range'] {
      width: 400px;
    }

    input[type='text'] {
      width: 50%;
      font-size: inherit;
      text-align: right;
      max-height: 25px;
    }

    input[type='text'].orientation {
      max-width: 50px;
    }

    input[type='button'] {
      display: inline;
      font-size: inherit;
      max-width: 200px;
    }
  `;

  connectedCallback() {
    super.connectedCallback(); // eslint-disable-line
    simulationState.registerObserver(this);
  }

  disconnectedCallback() {
    simulationState.removeObserver(this);
    super.disconnectedCallback(); // eslint-disable-line
  }

  onNotify(data: SimulationInfo) {
    if (data.selectedId && this.editMode === false) {
      for (const device of data.devices) {
        if (device.name === data.selectedId) {
          this.selectedDevice = device;
          if (!this.holdRange){
            this.yaw = device.orientation.yaw;
            this.pitch = device.orientation.pitch;
            this.roll = device.orientation.roll;
          }
          this.posX = Math.floor(device.position.x * 100);
          this.posY = Math.floor(device.position.y * 100);
          this.posZ = Math.floor(device.position.z * 100);
          break;
        }
      }
    }
  }

  private changeRange(ev: InputEvent) {
    this.holdRange = true;
    console.assert(this.selectedDevice !== null); // eslint-disable-line
    const range = ev.target as HTMLInputElement;
    const event = new CustomEvent('orientationEvent', {
      detail: {
        name: this.selectedDevice?.name,
        type: range.id,
        value: range.value,
      },
    });
    window.dispatchEvent(event);
    if (range.id === 'yaw') {
      this.yaw = Number(range.value);
    } else if (range.id === 'pitch') {
      this.pitch = Number(range.value);
    } else {
      this.roll = Number(range.value);
    }
  }

  private patchOrientation() {
    this.holdRange = false;
    console.assert(this.selectedDevice !== undefined); // eslint-disable-line
    if (this.selectedDevice === undefined) return;
    this.selectedDevice.orientation = {yaw: this.yaw, pitch: this.pitch, roll: this.roll};
    simulationState.patchDevice({
      device: {
        name: this.selectedDevice.name,
        orientation: this.selectedDevice.orientation,
      },
    });
  }

  private patchRadio() {
    console.assert(this.selectedDevice !== undefined); // eslint-disable-line
    if (this.selectedDevice === undefined) return;
    simulationState.patchDevice({
      device: {
        name: this.selectedDevice.name,
        chips: this.selectedDevice.chips,
      },
    });
  }

  private handleEditForm() {
    if (this.editMode) {
      simulationState.invokeGetDevice();
      this.editMode = false;
    } else {
      this.editMode = true;
    }
  }

  static checkPositionBound(value: number) {
    return value > 10 ? 10 : value < 0 ? 0 : value; // eslint-disable-line
  }

  static checkOrientationBound(value: number) {
    return value > 90 ? 90 : value < -90 ? -90 : value; // eslint-disable-line
  }

  private handleSave() {
    console.assert(this.selectedDevice !== undefined); // eslint-disable-line
    if (this.selectedDevice === undefined) return;
    const elements = this.renderRoot.querySelectorAll(`[id^="edit"]`);
    const obj: Record<string, any> = {
      name: this.selectedDevice.name,
      position: this.selectedDevice.position,
      orientation: this.selectedDevice.orientation,
    };
    elements.forEach(element => {
      const inputElement = element as HTMLInputElement;
      if (inputElement.id === 'editName') {
        obj.name = inputElement.value;
      } else if (inputElement.id.startsWith('editPos')) {
        if (!Number.isNaN(Number(inputElement.value))) {
          obj.position[inputElement.id.slice(7).toLowerCase()] =
            DeviceInformation.checkPositionBound(
              Number(inputElement.value) / 100
            );
        }
      } else if (inputElement.id.startsWith('editOri')) {
        if (!Number.isNaN(Number(inputElement.value))) {
          obj.orientation[inputElement.id.slice(7).toLowerCase()] =
            DeviceInformation.checkOrientationBound(Number(inputElement.value));
        }
      }
    });
    this.selectedDevice.name = obj.name;
    this.selectedDevice.position = obj.position;
    this.selectedDevice.orientation = obj.orientation;
    this.handleEditForm();
    simulationState.patchDevice({
      device: obj,
    });
  }

  private handleGetChips() {
    let lowEnergyCheckbox = html``;
    let classicCheckbox = html``;
    let wifiCheckbox = html``;
    let uwbCheckbox = html``;
    const disabledCheckbox = html`
      <input type="checkbox" disabled />
        <span
          class="slider round"
          style=${styleMap({ opacity: '0.7' })}
        ></span>
    `;
    if (this.selectedDevice) {
      if ("chips" in this.selectedDevice && this.selectedDevice.chips) {
        for (const chip of this.selectedDevice.chips) {
          if ('bt' in chip && chip.bt) {
            if ("lowEnergy" in chip.bt && chip.bt.lowEnergy && 'state' in chip.bt.lowEnergy) {
              lowEnergyCheckbox = html `
                <input
                  id="lowEnergy"
                  type="checkbox"
                  .checked=${live(chip.bt.lowEnergy.state === State.ON)}
                  @click=${() => {
                    // eslint-disable-next-line
                    this.selectedDevice?.toggleChipState(chip, "lowEnergy");
                    this.patchRadio();
                  }}
                />
                <span class="slider round"></span>
              `;
            } else {
              lowEnergyCheckbox = disabledCheckbox;
            }
            if ("classic" in chip.bt && chip.bt.classic && 'state' in chip.bt.classic) {
              classicCheckbox = html`
                <input
                  id="classic"
                  type="checkbox"
                  .checked=${live(chip.bt.classic.state === State.ON)}
                  @click=${() => {
                    // eslint-disable-next-line
                    this.selectedDevice?.toggleChipState(chip, "classic");
                    this.patchRadio();
                  }}
                />
                <span class="slider round"></span>
              `;
            } else {
              classicCheckbox = disabledCheckbox;
            }
          }

          if ('wifi' in chip && chip.wifi) {
            wifiCheckbox = html`
              <input
                id="wifi"
                type="checkbox"
                .checked=${live(chip.wifi.state === State.ON)}
                @click=${() => {
                  // eslint-disable-next-line
                  this.selectedDevice?.toggleChipState(chip);
                  this.patchRadio();
                }}
              />
              <span class="slider round"></span>
            `;
          } else {
            wifiCheckbox = disabledCheckbox;
          }

          if ('uwb' in chip && chip.uwb) {
            uwbCheckbox = html`
              <input
                id="uwb"
                type="checkbox"
                .checked=${live(chip.uwb.state === State.ON)}
                @click=${() => {
                  // eslint-disable-next-line
                  this.selectedDevice?.toggleChipState(chip);
                  this.patchRadio();
                }}
              />
              <span class="slider round"></span>
            `;
          } else {
            uwbCheckbox = disabledCheckbox;
          }
        }
      }
    }
    return html`
      <div class="label">BLE</div>
      <div class="info">
        <label class="switch">
          ${lowEnergyCheckbox}
        </label>
      </div>
      <div class="label">Classic</div>
      <div class="info">
        <label class="switch">
          ${classicCheckbox}
        </label>
      </div>
      <div class="label">WIFI</div>
      <div class="info">
        <label class="switch">
          ${wifiCheckbox}
        </label>
      </div>
      <div class="label">UWB</div>
      <div class="info">
        <label class="switch">
          ${uwbCheckbox}
        </label>
      </div>
    `;
  }

  render() {
    return html`${this.selectedDevice
      ? html`
          <div class="title">Device Info</div>
          <div class="setting">
            <div class="name">Name</div>
            <div class="info">${this.selectedDevice.name}</div>
          </div>
          <div class="setting">
            <div class="name">Position</div>
            <div class="label">X</div>
            <div class="info" style=${styleMap({ color: 'red' })}>
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editPosX"
                    .value=${this.posX.toString()}
                  />`
                : html`${this.posX}`}
            </div>
            <div class="label">Y</div>
            <div class="info" style=${styleMap({ color: 'green' })}>
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editPosY"
                    .value=${this.posY.toString()}
                  />`
                : html`${this.posY}`}
            </div>
            <div class="label">Z</div>
            <div class="info" style=${styleMap({ color: 'blue' })}>
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editPosZ"
                    .value=${this.posZ.toString()}
                  />`
                : html`${this.posZ}`}
            </div>
          </div>
          <div class="setting">
            <div class="name">Orientation</div>
            <div class="label">Yaw</div>
            <div class="info">
              <input
                id="yaw"
                type="range"
                min="-90"
                max="90"
                .value=${this.yaw.toString()}
                .disabled=${this.editMode}
                @input=${this.changeRange}
                @change=${this.patchOrientation}
              />
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editOriYaw"
                    class="orientation"
                    .value=${this.yaw.toString()}
                  />`
                : html`<div class="text">(${this.yaw})</div>`}
            </div>
            <div class="label">Pitch</div>
            <div class="info">
              <input
                id="pitch"
                type="range"
                min="-90"
                max="90"
                .value=${this.pitch.toString()}
                .disabled=${this.editMode}
                @input=${this.changeRange}
                @change=${this.patchOrientation}
              />
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editOriPitch"
                    class="orientation"
                    .value=${this.pitch.toString()}
                  />`
                : html`<div class="text">(${this.pitch})</div>`}
            </div>
            <div class="label">Roll</div>
            <div class="info">
              <input
                id="roll"
                type="range"
                min="-90"
                max="90"
                .value=${this.roll.toString()}
                .disabled=${this.editMode}
                @input=${this.changeRange}
                @change=${this.patchOrientation}
              />
              ${this.editMode
                ? html`<input
                    type="text"
                    id="editOriRoll"
                    class="orientation"
                    .value=${this.roll.toString()}
                  />`
                : html`<div class="text">(${this.roll})</div>`}
            </div>
          </div>
          <div class="setting">
            ${this.editMode
              ? html`
                  <input type="button" value="Save" @click=${this.handleSave} />
                  <input
                    type="button"
                    value="Cancel"
                    @click=${this.handleEditForm}
                  />
                `
              : html`<input
                  type="button"
                  value="Edit"
                  @click=${this.handleEditForm}
                />`}
          </div>
          <div class="setting">
            <div class="name">Radio States</div>
            ${this.handleGetChips()}
          </div>
        `
      : html`<div class="title">Device Info</div>`}`;
  }
}
