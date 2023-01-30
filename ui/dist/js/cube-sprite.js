import{__decorate as t}from"../node_modules/tslib/tslib.es6.js";import{css as e,LitElement as i,html as o}from"https://cdn.jsdelivr.net/gh/lit/dist@2/core/lit-core.min.js";import{property as r,customElement as s}from"https://cdn.skypack.dev/pin/lit@v2.5.0-jYRq0AKQogjUdUh7SCAE/mode=imports/optimized/lit/decorators.js";import{simulationState as d}from"./device-observer.js";let a=class extends i{constructor(){super(...arguments),this.yaw=-15,this.pitch=-15,this.roll=0,this.posZ=0,this.color=e`red`,this.size=e`30px`,this.controls=!1,this.highlighted=!1,this.handleOrientationEvent=t=>{const{detail:e}=t;e.deviceSerial===this.id&&this.controls&&("yaw"===e.type?this.yaw=e.value:"pitch"===e.type?this.pitch=e.value:this.roll=e.value)}}connectedCallback(){super.connectedCallback(),d.registerObserver(this),window.addEventListener("orientationEvent",this.handleOrientationEvent)}disconnectedCallback(){window.removeEventListener("orientationEvent",this.handleOrientationEvent),d.removeObserver(this),super.disconnectedCallback()}onNotify(t){this.highlighted=t.selectedSerial===this.id;for(const e of t.devices)if(e.deviceSerial===this.id)return void(e.position&&e.position.z?this.posZ=100*e.position.z:this.posZ=0)}render(){return o`
      <style>
        :host {
          font-size: ${this.size};
          --color: ${this.color};
          --yaw: ${this.yaw};
          --pitch: ${this.pitch};
          --roll: ${this.roll};
          --posZ: ${this.controls?this.posZ:0};
        }
        .cube > div {
          outline: ${this.highlighted&&this.controls?e`dashed`:e``};
        }
      </style>
      <div class="cube">
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
      </div>
      ${this.controls?o`
            <div class="line"></div>
            <div class="base"></div>
          `:o``}
    `}};a.styles=e`
    :host {
      /** all sizes are relative to font-size **/
      display: block;
      min-height: 1.5em;
      min-width: 1.5em;
      width: 1em;
      /*  overflow: hidden; */
      transform-origin: center;
      transform-style: preserve-3d;
      transform: translateZ(calc(var(--posZ) * 1px));
    }

    .cube {
      transform-style: preserve-3d;
      transform: rotateX(calc(var(--yaw) * 1deg))
        rotateY(calc(var(--pitch) * 1deg)) rotateZ(calc(var(--roll) * 1deg));
      position: absolute;
      left: 0.25em;
      bottom: 0.25em;
      width: 1em;
      height: 1em;
    }
    .cube > div {
      position: absolute;
      background-color: var(--color);
      width: 100%;
      height: 100%;
      box-shadow: 0 0 0.25em #000 inset;
    }
    .cube > div:nth-child(1) {
      transform: translateZ(0.5em);
    }
    .cube > div:nth-child(2) {
      transform: rotateY(180deg) translateZ(0.5em);
    }
    .cube > div:nth-child(3) {
      right: 0;
      width: 1em;
      transform: rotateY(90deg) translateZ(0.5em);
    }
    .cube > div:nth-child(4) {
      width: 1em;
      transform: rotateY(270deg) translateZ(0.5em);
    }
    .cube > div:nth-child(5) {
      bottom: -0.5em;
      height: 1em;
      transform: rotateX(90deg);
      box-shadow: 0 0 0.25em #000 inset, 0 0 0.25em #000;
    }
    .cube div:nth-child(6) {
      height: 1em;
      transform: translateY(-0.5em) rotateX(90deg);
      overflow: hidden;
    }

    .line {
      position: absolute;
      border-bottom: 5px dashed;
      width: calc(var(--posZ) * 1px);
      top: 50%;
      left: 50%;
      transform: rotateY(90deg) rotateX(90deg);
      transform-origin: left;
    }

    .base {
      position: absolute;
      border: 5px solid;
      border-radius: 50%;
      background-color: black;
      height: 5px;
      width: 5px;
      top: 50%;
      left: 50%;
      transform: translate3d(-50%, -50%, calc(var(--posZ) * -1px));
    }
  `,t([r({type:Number})],a.prototype,"yaw",void 0),t([r({type:Number})],a.prototype,"pitch",void 0),t([r({type:Number})],a.prototype,"roll",void 0),t([r({type:Number})],a.prototype,"posZ",void 0),t([r({type:e,attribute:"color"})],a.prototype,"color",void 0),t([r({type:e,attribute:"size"})],a.prototype,"size",void 0),t([r({type:Boolean})],a.prototype,"controls",void 0),t([r({type:Boolean})],a.prototype,"highlighted",void 0),a=t([s("ns-cube-sprite")],a);export{a as CubeSprite};
