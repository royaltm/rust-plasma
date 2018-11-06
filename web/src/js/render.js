const min = Math.min;
const abs = Math.abs;
const floor = Math.floor;
const PI2 = 2.0*Math.PI;
const PI05 = 0.5*Math.PI;
// const sin = Math.sin;
// const cos = Math.cos;

const SINTAB_LENGTH = 4096;
const sinTab = new Float32Array(SINTAB_LENGTH);

for (let i = 0; i < SINTAB_LENGTH; ++i) {
  sinTab[i] = Math.sin(PI2 * i / SINTAB_LENGTH);
}

function sin(v) {
  if (v < 0.0) {
      return -sinTab[((-v * SINTAB_LENGTH / PI2)|0) % SINTAB_LENGTH];
  }
  else {
      return sinTab[((v * SINTAB_LENGTH / PI2)|0) % SINTAB_LENGTH];
  }
}

function cos(v) {
    return sin(v - PI05);
}


export function render(buffer, ampls, pitch, pw, ph, x1, y1, w, h, offs) {
  var x2 = min(pw, x1 + w);
  var y2 = min(ph, y1 + h);
  var wr = pw / PI2;
  var hr = ph / PI2;
  for(let y = y1; y < y2; ++y) {
      let offset = y * pitch - offs;
      let yy = y / hr;
      for(let x = x1; x < x2; ++x) {
          put_pixel(buffer, offset, x / wr, yy, ampls);
          offset += 4;
      }
  }
}

function put_pixel(buffer, offset, x, y, ampls) {
    var hue0 = compose4(x, y, ampls.subarray(0, 16));
    var hue1 = compose4(x, y, ampls.subarray(16, 32));
    var sat = to_val(compose4(x, y, ampls.subarray(32, 48)));
    var rgb0 = hsv2rgb(hue0, 1.0, 1.0);
    var rgb1 = hsv2rgb(hue1, sat, 1.0);
    var red = rgb0.r - rgb1.r;
    var green = rgb0.g - rgb1.g;
    var blue = rgb0.b - rgb1.b;
    buffer[offset] = to_color8(red);
    buffer[offset + 1] = to_color8(green);
    buffer[offset + 2] = to_color8(blue);
    buffer[offset + 3] = 255;
}

function compose4(x, y, ampls) {
  return compose(x, ampls[0], ampls[1], ampls[2], ampls[3]) -
         compose(y, ampls[4], ampls[5], ampls[6], ampls[7]) *
         compose(x, ampls[8], ampls[9], ampls[10], ampls[11]) +
         compose(y, ampls[12], ampls[13], ampls[14], ampls[15]);
}

function compose(x, phase0, amplitude0, phase1, amplitude1) {
  var nor = amplitude0 + amplitude1;
  if (nor == 0.0) {
    return 0.0
  }
  else {
      return (
          sin(x + phase0)*amplitude0
        + cos(x + phase1)*amplitude1
      ) / nor;
  }
}

function to_color8(v) {
  v = abs(v) * 256.0;
  return (v > 255.0 ? 255 : v)>>>0;
}

function to_val(v) {
  v = abs(v);
  return (v > 1.0) ? 1.0 : v;
}

function hsv2rgb(hue, saturation, value) {
  var c = value * saturation;
  var h = (hue - (floor(hue / 2.0) * 2.0))*3;
  var x = c * (1.0 - abs(h % 2.0 - 1.0));
  var m = value - c;

  if (h >= 0.0 && h < 1.0) {
      return {r: c+m, g: x+m, b: m};
  } else if (h >= 1.0 && h < 2.0) {
      return {r: x+m, g: c+m, b: m};
  } else if (h >= 2.0 && h < 3.0) {
      return {r: m,   g: c+m, b: x+m};
  } else if (h >= 3.0 && h < 4.0) {
      return {r: m,   g: x+m, b: c+m};
  } else if (h >= 4.0 && h < 5.0) {
      return {r: x+m, g: m,   b: c+m};
  } else {
      return {r: c+m, g: m,   b: x+m};
  };
}
