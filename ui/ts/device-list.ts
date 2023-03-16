import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';

import {Device, Notifiable, SimulationInfo, simulationState,} from './device-observer.js';

@customElement('ns-device-list')
export class DeviceList extends LitElement implements Notifiable {
  @property() deviceData: Device[] = [];

  connectedCallback(): void {
    // eslint-disable-next-line
    super.connectedCallback();
    simulationState.registerObserver(this);
  }

  disconnectedCallback(): void {
    // eslint-disable-next-line
    super.disconnectedCallback();
    simulationState.removeObserver(this);
  }

  static styles = css`
    :host {
      justify-content: center;
      display: flex;
      flex-wrap: wrap;
      gap: 1rem;
      margin: 0;
      padding: 0;
      list-style: none;
    }

    li {
      border-style: solid;
      border-color: lightgray;
      flex-grow: 0;
      flex-shrink: 0;
      flex-basis: 125px;
    }

    li center {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      margin: 8px;
    }

    .box {
      position: relative;
      width: 80vw;
      height: 60vh;
      border: solid 1px rgb(198, 210, 255);
      margin: 2.5em auto;
    }
  `;

  onNotify(data: SimulationInfo): void {
    this.deviceData = data.devices;
    this.requestUpdate();
  }

  render() {
    const rainbow = [
      'red',
      'orange',
      'yellow',
      'green',
      'blue',
      'indigo',
      'purple',
    ];

    // Repeating templates with map
    return html`
      ${
        this.deviceData.map(
            (device, idx) => html`
          <li>
            <center>
              ${
                device.visible === true ?
                    html`<ns-cube-sprite
                      id=${device.name}
                      color=${rainbow[idx % rainbow.length]}
                      size="30px"
                      style="opacity:0.5;"
                    ></ns-cube-sprite
                    >${device.name} ` :
                    html`<ns-device-dragzone action="move">
                      <ns-cube-sprite
                        id=${device.name}
                        color=${rainbow[idx % rainbow.length]}
                        size="30px"
                      ></ns-cube-sprite> </ns-device-dragzone
                    >${device.name}`}
            </center>
          </li>
        `)}
      <li>
        <center>
          <ns-pyramid-sprite
            id="1234"
            color=${rainbow[this.deviceData.length % rainbow.length]}
            size="30px"
            style="opacity:0.5;"
          ></ns-pyramid-sprite
          >beacon
        </center>
      </li>
      <li>
        <center>
          <ns-pyramid-sprite
            id="5678"
            color=${rainbow[(this.deviceData.length + 1) % rainbow.length]}
            size="30px"
            style="opacity:0.5;"
          ></ns-pyramid-sprite
          >anchor
        </center>
      </li>
    `;
  }
}
