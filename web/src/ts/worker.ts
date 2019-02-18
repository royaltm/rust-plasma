import { PlasmaHandle } from '../../plasma_web';

// ugly hack
// see: https://stackoverflow.com/questions/50402004/error-ts2554-expected-2-3-arguments-but-got-1/50420456#50420456
const worker: Worker = self as any;

var plasma: PlasmaHandle;
var imageData: ImageData;

worker.postMessage("ready");

onmessage = (event) => {
    var data: any = event.data;
    if (data instanceof ArrayBuffer) {
        // plasma.importPhaseAmps(new Float32Array(data));
        // plasma.render();
        plasma.renderPhaseAmps(new Float32Array(data));
        if (imageData.data.byteLength === 0) {
            imageData = plasma.imageData();
        }
        // A workaround for Chrome/Opera. Since version 70 it can't create image bitmaps from memory shared with wasm.
        let img = new ImageData(new Uint8ClampedArray(imageData.data), imageData.width , imageData.height);
        createImageBitmap(img).then(imageBitmap => {
            worker.postMessage(imageBitmap, [imageBitmap]);
        }, (err: any) => {
            console.error(err);
        });
    }
    else {
        let { width, height, minSteps, maxSteps, x, y, w, h } = data;
        plasma = new PlasmaHandle(width, height, minSteps, maxSteps);
        plasma.setArea(x, y, w, h);
        imageData = plasma.imageData();
    }
};
