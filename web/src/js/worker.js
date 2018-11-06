import { PlasmaHandle } from '../../plasma_web';
import { render } from './render';

var plasma, imageData;

postMessage("ready");

// var pw, ph, x0, y0, w0, h0, pitch, offs;

onmessage = (event) => {
  var data = event.data;
  if (data instanceof ArrayBuffer) {
    // plasma.importPhaseAmps(new Float32Array(data));
    // plasma.render();
    plasma.renderPhaseAmps(new Float32Array(data));
    // render(imageData.data, new Float32Array(data), pitch, pw, ph, x0, y0, w0, h0, offs);
    createImageBitmap(imageData).then(imageBitmap => {
      postMessage(imageBitmap, [imageBitmap]);
    }, err => {
      console.error(err);
    });
  }
  else {
    let { width, height, minSteps, maxSteps, x, y, w, h } = data;
    plasma = new PlasmaHandle(width, height, minSteps, maxSteps);
    plasma.setArea(x, y, w, h);
    imageData = plasma.imageData();
    // pitch = 4 * w;
    // pw = width;
    // ph = height;
    // x0 = x;
    // y0 = y;
    // w0 = w;
    // h0 = h;
    // offs = y*pitch;
  }
};
