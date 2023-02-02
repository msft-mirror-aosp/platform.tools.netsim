import{__decorate as e}from"../node_modules/tslib/tslib.es6.js";import{css as i,LitElement as t,html as s}from"https://cdn.jsdelivr.net/gh/lit/dist@2/core/lit-core.min.js";import{property as o,customElement as d}from"https://cdn.skypack.dev/pin/lit@v2.5.0-jYRq0AKQogjUdUh7SCAE/mode=imports/optimized/lit/decorators.js";import{styleMap as l,live as a}from"https://cdn.jsdelivr.net/gh/lit/dist@2/all/lit-all.min.js";import{simulationState as n}from"./device-observer.js";var c;let r=c=class extends t{constructor(){super(...arguments),this.yaw=0,this.pitch=0,this.roll=0,this.editMode=!1,this.posX=0,this.posY=0,this.posZ=0,this.holdRange=!1}connectedCallback(){super.connectedCallback(),n.registerObserver(this)}disconnectedCallback(){n.removeObserver(this),super.disconnectedCallback()}onNotify(e){if(e.selectedSerial&&!1===this.editMode)for(const i of e.devices)if(i.deviceSerial===e.selectedSerial){this.selectedDevice=i,this.holdRange||(this.yaw=i.orientation.yaw,this.pitch=i.orientation.pitch,this.roll=i.orientation.roll),this.posX=Math.floor(100*i.position.x),this.posY=Math.floor(100*i.position.y),this.posZ=Math.floor(100*i.position.z);break}}changeRange(e){var i;this.holdRange=!0,console.assert(null!==this.selectedDevice);const t=e.target,s=new CustomEvent("orientationEvent",{detail:{deviceSerial:null===(i=this.selectedDevice)||void 0===i?void 0:i.deviceSerial,type:t.id,value:t.value}});window.dispatchEvent(s),"yaw"===t.id?this.yaw=Number(t.value):"pitch"===t.id?this.pitch=Number(t.value):this.roll=Number(t.value)}updateOrientation(){this.holdRange=!1,console.assert(void 0!==this.selectedDevice),void 0!==this.selectedDevice&&(this.selectedDevice.orientation={yaw:this.yaw,pitch:this.pitch,roll:this.roll},n.updateDevice({device:{deviceSerial:this.selectedDevice.deviceSerial,orientation:this.selectedDevice.orientation}}))}updateRadio(){console.assert(void 0!==this.selectedDevice),void 0!==this.selectedDevice&&n.updateDevice({device:{deviceSerial:this.selectedDevice.deviceSerial,chips:this.selectedDevice.chips}})}handleEditForm(){this.editMode?(n.invokeGetDevice(),this.editMode=!1):this.editMode=!0}static checkPositionBound(e){return e>10?10:e<0?0:e}static checkOrientationBound(e){return e>90?90:e<-90?-90:e}handleSave(){if(console.assert(void 0!==this.selectedDevice),void 0===this.selectedDevice)return;const e=this.renderRoot.querySelectorAll('[id^="edit"]'),i={deviceSerial:this.selectedDevice.deviceSerial,name:this.selectedDevice.name,position:this.selectedDevice.position,orientation:this.selectedDevice.orientation};e.forEach((e=>{const t=e;"editName"===t.id?i.name=t.value:t.id.startsWith("editPos")?Number.isNaN(Number(t.value))||(i.position[t.id.slice(7).toLowerCase()]=c.checkPositionBound(Number(t.value)/100)):t.id.startsWith("editOri")&&(Number.isNaN(Number(t.value))||(i.orientation[t.id.slice(7).toLowerCase()]=c.checkOrientationBound(Number(t.value))))})),this.selectedDevice.name=i.name,this.selectedDevice.position=i.position,this.selectedDevice.orientation=i.orientation,this.handleEditForm(),n.updateDevice({device:i})}handleGetChips(){let e=s``,i=s``,t=s``,o=s``;const d=s`
      <input type="checkbox" disabled />
        <span
          class="slider round"
          style=${l({opacity:"0.7"})}
        ></span>
    `;if(this.selectedDevice&&"chips"in this.selectedDevice&&this.selectedDevice.chips)for(const l of this.selectedDevice.chips)"bt"in l&&l.bt&&(e="lowEnergy"in l.bt&&l.bt.lowEnergy&&"state"in l.bt.lowEnergy?s`
                <input
                  id="lowEnergy"
                  type="checkbox"
                  .checked=${a("ON"===l.bt.lowEnergy.state)}
                  @click=${()=>{var e;null===(e=this.selectedDevice)||void 0===e||e.toggleChipState(l,"lowEnergy"),this.updateRadio()}}
                />
                <span class="slider round"></span>
              `:d,i="classic"in l.bt&&l.bt.classic&&"state"in l.bt.classic?s`
                <input
                  id="classic"
                  type="checkbox"
                  .checked=${a("ON"===l.bt.classic.state)}
                  @click=${()=>{var e;null===(e=this.selectedDevice)||void 0===e||e.toggleChipState(l,"classic"),this.updateRadio()}}
                />
                <span class="slider round"></span>
              `:d),t="wifi"in l&&l.wifi?s`
              <input
                id="wifi"
                type="checkbox"
                .checked=${a("ON"===l.wifi.state)}
                @click=${()=>{var e;null===(e=this.selectedDevice)||void 0===e||e.toggleChipState(l),this.updateRadio()}}
              />
              <span class="slider round"></span>
            `:d,o="uwb"in l&&l.uwb?s`
              <input
                id="uwb"
                type="checkbox"
                .checked=${a("ON"===l.uwb.state)}
                @click=${()=>{var e;null===(e=this.selectedDevice)||void 0===e||e.toggleChipState(l),this.updateRadio()}}
              />
              <span class="slider round"></span>
            `:d;return s`
      <div class="label">BLE</div>
      <div class="info">
        <label class="switch">
          ${e}
        </label>
      </div>
      <div class="label">Classic</div>
      <div class="info">
        <label class="switch">
          ${i}
        </label>
      </div>
      <div class="label">WIFI</div>
      <div class="info">
        <label class="switch">
          ${t}
        </label>
      </div>
      <div class="label">UWB</div>
      <div class="info">
        <label class="switch">
          ${o}
        </label>
      </div>
    `}render(){var e,i;return s`${this.selectedDevice?s`
          <div class="title">Device Info</div>
          <div class="setting">
            <div class="name">Name</div>
            <div class="info">
              ${this.editMode?s`<input
                    type="text"
                    id="editName"
                    .value=${null!==(e=this.selectedDevice.name)&&void 0!==e?e:""}
                  />`:s`${null!==(i=this.selectedDevice.name)&&void 0!==i?i:""}`}
            </div>
          </div>
          <div class="setting">
            <div class="name">Serial</div>
            <div class="info">${this.selectedDevice.deviceSerial}</div>
          </div>
          <div class="setting">
            <div class="name">Position</div>
            <div class="label">X</div>
            <div class="info" style=${l({color:"red"})}>
              ${this.editMode?s`<input
                    type="text"
                    id="editPosX"
                    .value=${this.posX.toString()}
                  />`:s`${this.posX}`}
            </div>
            <div class="label">Y</div>
            <div class="info" style=${l({color:"green"})}>
              ${this.editMode?s`<input
                    type="text"
                    id="editPosY"
                    .value=${this.posY.toString()}
                  />`:s`${this.posY}`}
            </div>
            <div class="label">Z</div>
            <div class="info" style=${l({color:"blue"})}>
              ${this.editMode?s`<input
                    type="text"
                    id="editPosZ"
                    .value=${this.posZ.toString()}
                  />`:s`${this.posZ}`}
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
                @change=${this.updateOrientation}
              />
              ${this.editMode?s`<input
                    type="text"
                    id="editOriYaw"
                    class="orientation"
                    .value=${this.yaw.toString()}
                  />`:s`<div class="text">(${this.yaw})</div>`}
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
                @change=${this.updateOrientation}
              />
              ${this.editMode?s`<input
                    type="text"
                    id="editOriPitch"
                    class="orientation"
                    .value=${this.pitch.toString()}
                  />`:s`<div class="text">(${this.pitch})</div>`}
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
                @change=${this.updateOrientation}
              />
              ${this.editMode?s`<input
                    type="text"
                    id="editOriRoll"
                    class="orientation"
                    .value=${this.roll.toString()}
                  />`:s`<div class="text">(${this.roll})</div>`}
            </div>
          </div>
          <div class="setting">
            ${this.editMode?s`
                  <input type="button" value="Save" @click=${this.handleSave} />
                  <input
                    type="button"
                    value="Cancel"
                    @click=${this.handleEditForm}
                  />
                `:s`<input
                  type="button"
                  value="Edit"
                  @click=${this.handleEditForm}
                />`}
          </div>
          <div class="setting">
            <div class="name">Radio States</div>
            ${this.handleGetChips()}
          </div>
        `:s`<div class="title">Device Info</div>`}`}};r.styles=i`
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
  `,e([o()],r.prototype,"selectedDevice",void 0),e([o({type:Number})],r.prototype,"yaw",void 0),e([o({type:Number})],r.prototype,"pitch",void 0),e([o({type:Number})],r.prototype,"roll",void 0),e([o({type:Boolean})],r.prototype,"editMode",void 0),e([o({type:Number})],r.prototype,"posX",void 0),e([o({type:Number})],r.prototype,"posY",void 0),e([o({type:Number})],r.prototype,"posZ",void 0),r=c=e([d("ns-device-info")],r);export{r as DeviceInformation};
