(window.webpackJsonpPlasma=window.webpackJsonpPlasma||[]).push([[1],[,function(e,t,r){"use strict";r.r(t),r.d(t,"DefaultPlasmaGeneratorOptions",function(){return _}),r.d(t,"DefaultCanvasRendererOptions",function(){return a}),r.d(t,"CanvasRenderer",function(){return i}),r.d(t,"PlasmaGenerator",function(){return o});var n=r(2);r.d(t,"PlasmaHandle",function(){return n.a});const _={minSteps:80,maxSteps:200,width:128,height:128,parallel:!0,workerUrl:"worker.js",workers:Math.min(2,Math.max(1,0|self.navigator.hardwareConcurrency))},a={width:256,height:256};class i{constructor(e,t){Object.assign(this,a,t),this.target=e,this.ctx=e.getContext("2d"),this.isAnimating=!1,this.spool=[],this.readyHandler=(()=>{let e=this.spool;0!==e.length&&(this.spool=[]),requestAnimationFrame(t=>{let r=this.ctx,n=this.target,_=this.width,a=this.height,i=n.width,o=n.height;if(e.forEach(e=>{let{x:t,y:n,width:c,height:u,bitmap:s}=e,l=s.width,p=s.height,b=_/c,f=a/u,d=t*b>>>0,g=l*b>>>0,m=p*f>>>0;for(let e=n*f>>>0;e<o;e+=a)for(let t=d;t<i;t+=_)r.drawImage(s,0,0,l,p,t,e,g,m);s.close()}),this.isAnimating){let e=this.generator;e&&e.render()}})}),this.bitmapHandler=(e=>{this.spool.push(e.detail)})}attach(e){this.animate(!1),this.detach(),this.generator=e,e.addEventListener("ready",this.readyHandler,!1),e.addEventListener("bitmap",this.bitmapHandler,!1)}detach(){var e=this.generator;return e&&(this.animate(!1),e.removeEventListener("ready",this.readyHandler,!1),e.removeEventListener("bitmap",this.bitmapHandler,!1)),this.generator=null,e}animate(e){if(!e!=!this.isAnimating)if(e){this.isAnimating=!0;let e=this.generator;e&&e.isReady&&e.render()}else this.isAnimating=!1}}class o extends EventTarget{constructor(e){const t=Object.assign({},_,e);super();const r=this.handle=new n.a(t.width,t.height,t.minSteps,t.maxSteps);if(this.workers=[],this.queued=-1,this.imageData=null,t.parallel){const[e,n]=[r.width(),r.height()],_=Math.max(1,Math.min(n,t.workers)),a=(n+_-1)/_>>>0;Array.from({length:_},()=>new Worker(t.workerUrl)).forEach((t,i)=>{const o=i*a,c={width:e,height:n,minSteps:r.minSteps(),maxSteps:r.maxSteps(),x:0,y:o,w:e,h:Math.min(a,n-o)};t.onmessage=(a=>{const i=a.data;if("ready"===i)t.postMessage(c),this.workers.push(t)==_&&(this.queued=0,this.dispatchEvent(new Event("ready")));else if(i instanceof ImageBitmap){let t={x:0,y:o,width:e,height:n,bitmap:i};this.dispatchEvent(new CustomEvent("bitmap",{detail:t})),0==--this.queued&&(r.update(),this.dispatchEvent(new Event("ready")))}else this.dispatchEvent(new CustomEvent("error",{detail:new Error("Unknown message from worker")}))})})}else setTimeout(()=>{this.queued=0,this.imageData=r.imageData(),this.dispatchEvent(new Event("ready"))},1)}get isReady(){return 0===this.queued}render(){if(0!==this.queued)return!1;const e=this.workers;let t=e.length;const r=this.handle;if(t>0){this.queued=t;let{buffer:n}=r.exportPhaseAmps();e.forEach((e,r)=>{let _=r+1<t?n.slice(0):n;e.postMessage(_,[_])})}else this.queued=-1,r.render(),createImageBitmap(this.imageData).then(e=>{let{width:t,height:n}=e,_={x:0,y:0,width:t,height:n,bitmap:e};this.dispatchEvent(new CustomEvent("bitmap",{detail:_})),r.update(),this.queued=0,this.dispatchEvent(new Event("ready"))},e=>{this.dispatchEvent(new CustomEvent("error",{detail:e}))});return!0}}},function(module,__webpack_exports__,__webpack_require__){"use strict";__webpack_require__.d(__webpack_exports__,"w",function(){return __widl_f_new_with_u8_clamped_array_ImageData}),__webpack_require__.d(__webpack_exports__,"x",function(){return __widl_instanceof_Window}),__webpack_require__.d(__webpack_exports__,"u",function(){return __widl_f_create_image_bitmap_with_image_data_Window}),__webpack_require__.d(__webpack_exports__,"y",function(){return __widl_instanceof_WorkerGlobalScope}),__webpack_require__.d(__webpack_exports__,"v",function(){return __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope}),__webpack_require__.d(__webpack_exports__,"e",function(){return __wbg_eval_bb6e17e51caa3e94}),__webpack_require__.d(__webpack_exports__,"i",function(){return __wbg_new_dd4c3522b45b0e21}),__webpack_require__.d(__webpack_exports__,"j",function(){return __wbg_newnoargs_96cbdf0d056b2fa8}),__webpack_require__.d(__webpack_exports__,"c",function(){return __wbg_call_ee8306f6b79399de}),__webpack_require__.d(__webpack_exports__,"h",function(){return __wbg_new_baf10398b0d0c64d}),__webpack_require__.d(__webpack_exports__,"b",function(){return __wbg_call_173f04c850a68d5f}),__webpack_require__.d(__webpack_exports__,"m",function(){return __wbg_self_58232ab37cbe6608}),__webpack_require__.d(__webpack_exports__,"d",function(){return __wbg_crypto_329b714d7e7d321d}),__webpack_require__.d(__webpack_exports__,"f",function(){return __wbg_getRandomValues_2f960218fce3a102}),__webpack_require__.d(__webpack_exports__,"g",function(){return __wbg_getRandomValues_5581e85fc6616df6}),__webpack_require__.d(__webpack_exports__,"l",function(){return __wbg_require_4a70cbfd3adc73a8}),__webpack_require__.d(__webpack_exports__,"k",function(){return __wbg_randomFillSync_355c3fcfa754fa4e}),__webpack_require__.d(__webpack_exports__,"a",function(){return PlasmaHandle}),__webpack_require__.d(__webpack_exports__,"q",function(){return __wbindgen_object_clone_ref}),__webpack_require__.d(__webpack_exports__,"r",function(){return __wbindgen_object_drop_ref}),__webpack_require__.d(__webpack_exports__,"o",function(){return __wbindgen_is_undefined}),__webpack_require__.d(__webpack_exports__,"n",function(){return __wbindgen_boolean_get}),__webpack_require__.d(__webpack_exports__,"p",function(){return __wbindgen_jsval_eq}),__webpack_require__.d(__webpack_exports__,"s",function(){return __wbindgen_rethrow}),__webpack_require__.d(__webpack_exports__,"t",function(){return __wbindgen_throw});var _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__=__webpack_require__(3);let cachegetFloat32Memory=null;function getFloat32Memory(){return null!==cachegetFloat32Memory&&cachegetFloat32Memory.buffer===_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer||(cachegetFloat32Memory=new Float32Array(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer)),cachegetFloat32Memory}function passArrayF32ToWasm(e){const t=_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.d(4*e.length);return getFloat32Memory().set(e,t/4),[t,e.length]}const stack=[],slab=[{obj:void 0},{obj:null},{obj:!0},{obj:!1}];function getObject(e){if(1==(1&e))return stack[e>>1];return slab[e>>1].obj}let slab_next=slab.length;function dropRef(e){if((e>>=1)<4)return;let t=slab[e];t.cnt-=1,t.cnt>0||(slab[e]=slab_next,slab_next=e)}function takeObject(e){const t=getObject(e);return dropRef(e),t}function getArrayF32FromWasm(e,t){return getFloat32Memory().subarray(e/4,e/4+t)}let cachedGlobalArgumentPtr=null;function globalArgumentPtr(){return null===cachedGlobalArgumentPtr&&(cachedGlobalArgumentPtr=_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.c()),cachedGlobalArgumentPtr}let cachegetUint32Memory=null;function getUint32Memory(){return null!==cachegetUint32Memory&&cachegetUint32Memory.buffer===_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer||(cachegetUint32Memory=new Uint32Array(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer)),cachegetUint32Memory}function addHeapObject(e){slab_next===slab.length&&slab.push(slab.length+1);const t=slab_next,r=slab[t];return slab_next=r,slab[t]={obj:e,cnt:1},t<<1}let cachegetUint8ClampedMemory=null;function getUint8ClampedMemory(){return null!==cachegetUint8ClampedMemory&&cachegetUint8ClampedMemory.buffer===_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer||(cachegetUint8ClampedMemory=new Uint8ClampedArray(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer)),cachegetUint8ClampedMemory}function getClampedArrayU8FromWasm(e,t){return getUint8ClampedMemory().subarray(e/1,e/1+t)}function __widl_f_new_with_u8_clamped_array_ImageData(e,t,r,n){let _=getClampedArrayU8FromWasm(e,t);try{return addHeapObject(new ImageData(_,r))}catch(e){const t=getUint32Memory();t[n/4]=1,t[n/4+1]=addHeapObject(e)}}function __widl_instanceof_Window(e){return getObject(e)instanceof Window?1:0}function __widl_f_create_image_bitmap_with_image_data_Window(e,t,r){try{return addHeapObject(getObject(e).createImageBitmap(getObject(t)))}catch(e){const t=getUint32Memory();t[r/4]=1,t[r/4+1]=addHeapObject(e)}}function __widl_instanceof_WorkerGlobalScope(e){return getObject(e)instanceof WorkerGlobalScope?1:0}const __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope_target="undefined"==typeof WorkerGlobalScope?null:WorkerGlobalScope.prototype.createImageBitmap||function(){throw new Error("wasm-bindgen: WorkerGlobalScope.createImageBitmap does not exist")};function __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope(e,t,r){try{return addHeapObject(__widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope_target.call(getObject(e),getObject(t)))}catch(e){const t=getUint32Memory();t[r/4]=1,t[r/4+1]=addHeapObject(e)}}const lTextDecoder="undefined"==typeof TextDecoder?__webpack_require__(4).TextDecoder:TextDecoder;let cachedTextDecoder=new lTextDecoder("utf-8"),cachegetUint8Memory=null;function getUint8Memory(){return null!==cachegetUint8Memory&&cachegetUint8Memory.buffer===_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer||(cachegetUint8Memory=new Uint8Array(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.e.buffer)),cachegetUint8Memory}function getStringFromWasm(e,t){return cachedTextDecoder.decode(getUint8Memory().subarray(e,e+t))}function __wbg_eval_bb6e17e51caa3e94(arg0,arg1,exnptr){let varg0=getStringFromWasm(arg0,arg1);try{return addHeapObject(eval(varg0))}catch(e){const t=getUint32Memory();t[exnptr/4]=1,t[exnptr/4+1]=addHeapObject(e)}}function __wbg_new_dd4c3522b45b0e21(e,t){let r=getStringFromWasm(e,t);return addHeapObject(new Error(r))}function __wbg_newnoargs_96cbdf0d056b2fa8(e,t){let r=getStringFromWasm(e,t);return addHeapObject(new Function(r))}function __wbg_call_ee8306f6b79399de(e,t,r){try{return addHeapObject(getObject(e).call(getObject(t)))}catch(e){const t=getUint32Memory();t[r/4]=1,t[r/4+1]=addHeapObject(e)}}function __wbg_new_baf10398b0d0c64d(e,t){let r=getStringFromWasm(e,t);return addHeapObject(new Function(r))}function __wbg_call_173f04c850a68d5f(e,t){return addHeapObject(getObject(e).call(getObject(t)))}function __wbg_self_58232ab37cbe6608(e){return addHeapObject(getObject(e).self)}function __wbg_crypto_329b714d7e7d321d(e){return addHeapObject(getObject(e).crypto)}function __wbg_getRandomValues_2f960218fce3a102(e){return addHeapObject(getObject(e).getRandomValues)}function getArrayU8FromWasm(e,t){return getUint8Memory().subarray(e/1,e/1+t)}function __wbg_getRandomValues_5581e85fc6616df6(e,t,r){let n=getArrayU8FromWasm(t,r);getObject(e).getRandomValues(n)}function __wbg_require_4a70cbfd3adc73a8(e,t){let r=getStringFromWasm(e,t);return addHeapObject(__webpack_require__(9)(r))}function __wbg_randomFillSync_355c3fcfa754fa4e(e,t,r){let n=getArrayU8FromWasm(t,r);getObject(e).randomFillSync(n)}function freePlasmaHandle(e){_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.a(e)}class PlasmaHandle{free(){const e=this.ptr;this.ptr=0,freePlasmaHandle(e)}constructor(e,t,r,n){this.ptr=_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.f(e,t,r,n)}setArea(e,t,r,n){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.p(this.ptr,e,t,r,n)}width(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.r(this.ptr)}height(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.i(this.ptr)}render(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.n(this.ptr)}renderPhaseAmps(e){const[t,r]=passArrayF32ToWasm(e);try{return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.o(this.ptr,t,r)}finally{_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.b(t,4*r)}}update(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.q(this.ptr)}imageData(){return takeObject(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.j(this.ptr))}createImageBitmap(){return takeObject(_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.g(this.ptr))}exportPhaseAmps(){const e=globalArgumentPtr();_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.h(e,this.ptr);const t=getUint32Memory(),r=t[e/4],n=t[e/4+1],_=getArrayF32FromWasm(r,n).slice();return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.b(r,4*n),_}importPhaseAmps(e){const[t,r]=passArrayF32ToWasm(e);try{return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.k(this.ptr,t,r)}finally{_plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.b(t,4*r)}}minSteps(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.m(this.ptr)}maxSteps(){return _plasma_web_bg__WEBPACK_IMPORTED_MODULE_0__.l(this.ptr)}}function __wbindgen_object_clone_ref(e){if(1==(1&e))return addHeapObject(getObject(e));return slab[e>>1].cnt+=1,e}function __wbindgen_object_drop_ref(e){dropRef(e)}function __wbindgen_is_undefined(e){return void 0===getObject(e)?1:0}function __wbindgen_boolean_get(e){let t=getObject(e);return"boolean"==typeof t?t?1:0:2}function __wbindgen_jsval_eq(e,t){return getObject(e)===getObject(t)?1:0}function __wbindgen_rethrow(e){throw takeObject(e)}function __wbindgen_throw(e,t){throw new Error(getStringFromWasm(e,t))}},function(e,t,r){"use strict";var n=r.w[e.i];e.exports=n;r(2);n.s()},function(e,t,r){(function(e,n){var _=/%[sdj%]/g;t.format=function(e){if(!m(e)){for(var t=[],r=0;r<arguments.length;r++)t.push(o(arguments[r]));return t.join(" ")}r=1;for(var n=arguments,a=n.length,i=String(e).replace(_,function(e){if("%%"===e)return"%";if(r>=a)return e;switch(e){case"%s":return String(n[r++]);case"%d":return Number(n[r++]);case"%j":try{return JSON.stringify(n[r++])}catch(e){return"[Circular]"}default:return e}}),c=n[r];r<a;c=n[++r])d(c)||!y(c)?i+=" "+c:i+=" "+o(c);return i},t.deprecate=function(r,_){if(h(e.process))return function(){return t.deprecate(r,_).apply(this,arguments)};if(!0===n.noDeprecation)return r;var a=!1;return function(){if(!a){if(n.throwDeprecation)throw new Error(_);n.traceDeprecation?console.trace(_):console.error(_),a=!0}return r.apply(this,arguments)}};var a,i={};function o(e,r){var n={seen:[],stylize:u};return arguments.length>=3&&(n.depth=arguments[2]),arguments.length>=4&&(n.colors=arguments[3]),f(r)?n.showHidden=r:r&&t._extend(n,r),h(n.showHidden)&&(n.showHidden=!1),h(n.depth)&&(n.depth=2),h(n.colors)&&(n.colors=!1),h(n.customInspect)&&(n.customInspect=!0),n.colors&&(n.stylize=c),s(n,e,n.depth)}function c(e,t){var r=o.styles[t];return r?"["+o.colors[r][0]+"m"+e+"["+o.colors[r][1]+"m":e}function u(e,t){return e}function s(e,r,n){if(e.customInspect&&r&&M(r.inspect)&&r.inspect!==t.inspect&&(!r.constructor||r.constructor.prototype!==r)){var _=r.inspect(n,e);return m(_)||(_=s(e,_,n)),_}var a=function(e,t){if(h(t))return e.stylize("undefined","undefined");if(m(t)){var r="'"+JSON.stringify(t).replace(/^"|"$/g,"").replace(/'/g,"\\'").replace(/\\"/g,'"')+"'";return e.stylize(r,"string")}if(g(t))return e.stylize(""+t,"number");if(f(t))return e.stylize(""+t,"boolean");if(d(t))return e.stylize("null","null")}(e,r);if(a)return a;var i=Object.keys(r),o=function(e){var t={};return e.forEach(function(e,r){t[e]=!0}),t}(i);if(e.showHidden&&(i=Object.getOwnPropertyNames(r)),E(r)&&(i.indexOf("message")>=0||i.indexOf("description")>=0))return l(r);if(0===i.length){if(M(r)){var c=r.name?": "+r.name:"";return e.stylize("[Function"+c+"]","special")}if(w(r))return e.stylize(RegExp.prototype.toString.call(r),"regexp");if(O(r))return e.stylize(Date.prototype.toString.call(r),"date");if(E(r))return l(r)}var u,y="",k=!1,v=["{","}"];(b(r)&&(k=!0,v=["[","]"]),M(r))&&(y=" [Function"+(r.name?": "+r.name:"")+"]");return w(r)&&(y=" "+RegExp.prototype.toString.call(r)),O(r)&&(y=" "+Date.prototype.toUTCString.call(r)),E(r)&&(y=" "+l(r)),0!==i.length||k&&0!=r.length?n<0?w(r)?e.stylize(RegExp.prototype.toString.call(r),"regexp"):e.stylize("[Object]","special"):(e.seen.push(r),u=k?function(e,t,r,n,_){for(var a=[],i=0,o=t.length;i<o;++i)D(t,String(i))?a.push(p(e,t,r,n,String(i),!0)):a.push("");return _.forEach(function(_){_.match(/^\d+$/)||a.push(p(e,t,r,n,_,!0))}),a}(e,r,n,o,i):i.map(function(t){return p(e,r,n,o,t,k)}),e.seen.pop(),function(e,t,r){if(e.reduce(function(e,t){return 0,t.indexOf("\n")>=0&&0,e+t.replace(/\u001b\[\d\d?m/g,"").length+1},0)>60)return r[0]+(""===t?"":t+"\n ")+" "+e.join(",\n  ")+" "+r[1];return r[0]+t+" "+e.join(", ")+" "+r[1]}(u,y,v)):v[0]+y+v[1]}function l(e){return"["+Error.prototype.toString.call(e)+"]"}function p(e,t,r,n,_,a){var i,o,c;if((c=Object.getOwnPropertyDescriptor(t,_)||{value:t[_]}).get?o=c.set?e.stylize("[Getter/Setter]","special"):e.stylize("[Getter]","special"):c.set&&(o=e.stylize("[Setter]","special")),D(n,_)||(i="["+_+"]"),o||(e.seen.indexOf(c.value)<0?(o=d(r)?s(e,c.value,null):s(e,c.value,r-1)).indexOf("\n")>-1&&(o=a?o.split("\n").map(function(e){return"  "+e}).join("\n").substr(2):"\n"+o.split("\n").map(function(e){return"   "+e}).join("\n")):o=e.stylize("[Circular]","special")),h(i)){if(a&&_.match(/^\d+$/))return o;(i=JSON.stringify(""+_)).match(/^"([a-zA-Z_][a-zA-Z_0-9]*)"$/)?(i=i.substr(1,i.length-2),i=e.stylize(i,"name")):(i=i.replace(/'/g,"\\'").replace(/\\"/g,'"').replace(/(^"|"$)/g,"'"),i=e.stylize(i,"string"))}return i+": "+o}function b(e){return Array.isArray(e)}function f(e){return"boolean"==typeof e}function d(e){return null===e}function g(e){return"number"==typeof e}function m(e){return"string"==typeof e}function h(e){return void 0===e}function w(e){return y(e)&&"[object RegExp]"===k(e)}function y(e){return"object"==typeof e&&null!==e}function O(e){return y(e)&&"[object Date]"===k(e)}function E(e){return y(e)&&("[object Error]"===k(e)||e instanceof Error)}function M(e){return"function"==typeof e}function k(e){return Object.prototype.toString.call(e)}function v(e){return e<10?"0"+e.toString(10):e.toString(10)}t.debuglog=function(e){if(h(a)&&(a=n.env.NODE_DEBUG||""),e=e.toUpperCase(),!i[e])if(new RegExp("\\b"+e+"\\b","i").test(a)){var r=n.pid;i[e]=function(){var n=t.format.apply(t,arguments);console.error("%s %d: %s",e,r,n)}}else i[e]=function(){};return i[e]},t.inspect=o,o.colors={bold:[1,22],italic:[3,23],underline:[4,24],inverse:[7,27],white:[37,39],grey:[90,39],black:[30,39],blue:[34,39],cyan:[36,39],green:[32,39],magenta:[35,39],red:[31,39],yellow:[33,39]},o.styles={special:"cyan",number:"yellow",boolean:"yellow",undefined:"grey",null:"bold",string:"green",date:"magenta",regexp:"red"},t.isArray=b,t.isBoolean=f,t.isNull=d,t.isNullOrUndefined=function(e){return null==e},t.isNumber=g,t.isString=m,t.isSymbol=function(e){return"symbol"==typeof e},t.isUndefined=h,t.isRegExp=w,t.isObject=y,t.isDate=O,t.isError=E,t.isFunction=M,t.isPrimitive=function(e){return null===e||"boolean"==typeof e||"number"==typeof e||"string"==typeof e||"symbol"==typeof e||void 0===e},t.isBuffer=r(7);var j=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];function D(e,t){return Object.prototype.hasOwnProperty.call(e,t)}t.log=function(){var e,r;console.log("%s - %s",(e=new Date,r=[v(e.getHours()),v(e.getMinutes()),v(e.getSeconds())].join(":"),[e.getDate(),j[e.getMonth()],r].join(" ")),t.format.apply(t,arguments))},t.inherits=r(8),t._extend=function(e,t){if(!t||!y(t))return e;for(var r=Object.keys(t),n=r.length;n--;)e[r[n]]=t[r[n]];return e}}).call(this,r(5),r(6))},function(e,t){var r;r=function(){return this}();try{r=r||Function("return this")()||(0,eval)("this")}catch(e){"object"==typeof window&&(r=window)}e.exports=r},function(e,t){var r,n,_=e.exports={};function a(){throw new Error("setTimeout has not been defined")}function i(){throw new Error("clearTimeout has not been defined")}function o(e){if(r===setTimeout)return setTimeout(e,0);if((r===a||!r)&&setTimeout)return r=setTimeout,setTimeout(e,0);try{return r(e,0)}catch(t){try{return r.call(null,e,0)}catch(t){return r.call(this,e,0)}}}!function(){try{r="function"==typeof setTimeout?setTimeout:a}catch(e){r=a}try{n="function"==typeof clearTimeout?clearTimeout:i}catch(e){n=i}}();var c,u=[],s=!1,l=-1;function p(){s&&c&&(s=!1,c.length?u=c.concat(u):l=-1,u.length&&b())}function b(){if(!s){var e=o(p);s=!0;for(var t=u.length;t;){for(c=u,u=[];++l<t;)c&&c[l].run();l=-1,t=u.length}c=null,s=!1,function(e){if(n===clearTimeout)return clearTimeout(e);if((n===i||!n)&&clearTimeout)return n=clearTimeout,clearTimeout(e);try{n(e)}catch(t){try{return n.call(null,e)}catch(t){return n.call(this,e)}}}(e)}}function f(e,t){this.fun=e,this.array=t}function d(){}_.nextTick=function(e){var t=new Array(arguments.length-1);if(arguments.length>1)for(var r=1;r<arguments.length;r++)t[r-1]=arguments[r];u.push(new f(e,t)),1!==u.length||s||o(b)},f.prototype.run=function(){this.fun.apply(null,this.array)},_.title="browser",_.browser=!0,_.env={},_.argv=[],_.version="",_.versions={},_.on=d,_.addListener=d,_.once=d,_.off=d,_.removeListener=d,_.removeAllListeners=d,_.emit=d,_.prependListener=d,_.prependOnceListener=d,_.listeners=function(e){return[]},_.binding=function(e){throw new Error("process.binding is not supported")},_.cwd=function(){return"/"},_.chdir=function(e){throw new Error("process.chdir is not supported")},_.umask=function(){return 0}},function(e,t){e.exports=function(e){return e&&"object"==typeof e&&"function"==typeof e.copy&&"function"==typeof e.fill&&"function"==typeof e.readUInt8}},function(e,t){"function"==typeof Object.create?e.exports=function(e,t){e.super_=t,e.prototype=Object.create(t.prototype,{constructor:{value:e,enumerable:!1,writable:!0,configurable:!0}})}:e.exports=function(e,t){e.super_=t;var r=function(){};r.prototype=t.prototype,e.prototype=new r,e.prototype.constructor=e}},function(e,t){function r(e){var t=new Error("Cannot find module '"+e+"'");throw t.code="MODULE_NOT_FOUND",t}r.keys=function(){return[]},r.resolve=r,e.exports=r,r.id=9}]]);