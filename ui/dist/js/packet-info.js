import{__decorate as t}from"../node_modules/tslib/tslib.es6.js";import{css as e,LitElement as i,html as o}from"https://cdn.jsdelivr.net/gh/lit/dist@2/core/lit-core.min.js";import{property as a,customElement as d}from"https://cdn.skypack.dev/pin/lit@v2.5.0-jYRq0AKQogjUdUh7SCAE/mode=imports/optimized/lit/decorators.js";import{simulationState as l}from"./device-observer.js";let n=class extends i{constructor(){super(...arguments),this.deviceData=[]}connectedCallback(){super.connectedCallback(),l.registerObserver(this)}disconnectedCallback(){l.removeObserver(this),super.disconnectedCallback()}onNotify(t){this.deviceData=t.devices,this.requestUpdate()}handleGetChips(t){var e,i,a,d,l,n,r,s;let c=o``,p=o``,h=o``;if("chips"in t&&t.chips)for(const b of t.chips){if("bt"in b&&b.bt){let t=o``,l=o``;"lowEnergy"in b.bt&&b.bt.lowEnergy&&(t=o`
              <tr>
                <td>BLE</td>
                <td>${null!==(e=b.bt.lowEnergy.rxCount)&&void 0!==e?e:0}</td>
                <td>${null!==(i=b.bt.lowEnergy.txCount)&&void 0!==i?i:0}</td>
              </tr>
            `),"classic"in b.bt&&b.bt.classic&&(l=o`
              <tr>
                <td>Bluetooth Classic</td>
                <td>${null!==(a=b.bt.classic.rxCount)&&void 0!==a?a:0}</td>
                <td>${null!==(d=b.bt.classic.txCount)&&void 0!==d?d:0}</td>
              </tr>
            `),c=o`${t} ${l}`}"uwb"in b&&b.uwb&&(p=o`
            <tr>
              <td>UWB</td>
              <td>${null!==(l=b.uwb.rxCount)&&void 0!==l?l:0}</td>
              <td>${null!==(n=b.uwb.txCount)&&void 0!==n?n:0}</td>
            </tr>
          `),"wifi"in b&&b.wifi&&(h=o`
            <tr>
              <td>WIFI</td>
              <td>${null!==(r=b.wifi.rxCount)&&void 0!==r?r:0}</td>
              <td>${null!==(s=b.wifi.txCount)&&void 0!==s?s:0}</td>
            </tr>
          `)}return o`
      ${c}
      ${p}
      ${h}
    `}handleGetCapture(t){let e=o``;if("chips"in t&&t.chips)for(const i of t.chips)e=o`
          ${e}
          <tr>
            <td>${t.name}</td>
            <td>${t.deviceSerial}</td>
            <td>
              ${i.bt?"Bluetooth":i.uwb?"UWB":i.wifi?"WIFI":"Unknown"}
            </td>
            <td>
              <input
                type="checkbox"
                class="switch_1"
                .checked=${"ON"===i.capture}
                @click=${()=>{t.toggleCapture(t,i)}}
              />
            </td>
            <td>
              <a
                href="http://localhost:3000/${t.deviceSerial}-hci.pcap"
                target="_blank"
                type="application/vnd.tcpdump.pcap"
                >Download PCAP</a
              >
            </td>
          </tr>
        `;return e}render(){return o`
      <div class="panel">
        <div class="title">Packet Info</div>
        ${this.deviceData.map((t=>o`
              <div class="label">${t.name} | ${t.deviceSerial}</div>
              <table class="styled-table">
                <tr>
                  <th>Radio</th>
                  <th>RX Count</th>
                  <th>TX Count</th>
                </tr>
                ${this.handleGetChips(t)}
              </table>
            `))}
        <div class="title">Packet Capture</div>
        <table class="styled-table">
          <tr>
            <th>Name</th>
            <th>Serial</th>
            <th>Chip Type</th>
            <th>Capture ON/OFF</th>
            <th>Packet Trace</th>
          </tr>
          ${this.deviceData.map((t=>this.handleGetCapture(t)))}
        </table>
      </div>
    `}};n.styles=e`
    .panel {
      cursor: pointer;
      display: grid;
      place-content: center;
      color: black;
      font-size: 25px;
      font-family: 'Lato', sans-serif;
      border: 5px solid black;
      border-radius: 12px;
      padding: 10px;
      background-color: #ffffff;
      max-width: max-content;
      float: left;
    }

    .title {
      font-weight: bold;
      text-transform: uppercase;
      text-align: center;
      margin-bottom: 10px;
    }

    .label {
      text-align: left;
    }

    .styled-table {
      border-collapse: collapse;
      margin: 25px 0;
      font-size: 20px;
      font-family: sans-serif;
      width: 100%;
      box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
    }

    .styled-table thead tr {
      background-color: #009879;
      color: #ffffff;
      text-align: left;
    }

    .styled-table th,
    .styled-table td {
      padding: 12px 15px;
      text-align: left;
    }

    .styled-table tbody tr {
      border-bottom: 1px solid #dddddd;
    }

    .styled-table tbody tr:nth-of-type(even) {
      background-color: #cac0c0;
    }

    input[type='button'] {
      height: 2rem;
      font-size: inherit;
    }

    input[type='checkbox'].switch_1 {
      font-size: 30px;
      -webkit-appearance: none;
      -moz-appearance: none;
      appearance: none;
      width: 3.5em;
      height: 1.5em;
      background: #ddd;
      border-radius: 3em;
      position: relative;
      cursor: pointer;
      outline: none;
      -webkit-transition: all 0.2s ease-in-out;
      transition: all 0.2s ease-in-out;
    }

    input[type='checkbox'].switch_1:checked {
      background: #0ebeff;
    }

    input[type='checkbox'].switch_1:after {
      position: absolute;
      content: '';
      width: 1.5em;
      height: 1.5em;
      border-radius: 50%;
      background: #fff;
      -webkit-box-shadow: 0 0 0.25em rgba(0, 0, 0, 0.3);
      box-shadow: 0 0 0.25em rgba(0, 0, 0, 0.3);
      -webkit-transform: scale(0.7);
      transform: scale(0.7);
      left: 0;
      -webkit-transition: all 0.2s ease-in-out;
      transition: all 0.2s ease-in-out;
    }

    input[type='checkbox'].switch_1:checked:after {
      left: calc(100% - 1.5em);
    }
  `,t([a()],n.prototype,"deviceData",void 0),n=t([d("ns-packet-info")],n);export{n as PacketInformation};
