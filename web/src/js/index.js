// import { memory } from '../plasma_web_bg';
import { PlasmaHandle } from '../../plasma_web';

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const TARGET_WIDTH = 900;
const TARGET_HEIGHT = 900;
const plasma = new PlasmaHandle(450, 450, 80, 200);

function resize() {
    var w = canvas.width = canvas.offsetWidth;
    var h = canvas.height = canvas.offsetHeight;
    ctx.imageSmoothingEnabled = false;
}
resize();
window.addEventListener('resize', resize, false);

const numworkers = Math.max(1, window.navigator.hardwareConcurrency|0);
const segmh = (plasma.height() + numworkers - 1) / numworkers >>> 0;
const sink = [];

var last_ts = performance.now(), frame_count = 0;

Array.from({length: numworkers}, () => new Worker("worker.js"))
.forEach((worker, i) => {
  const opts = {
    width: plasma.width(), height: plasma.height(),
    minSteps: plasma.minSteps(),
    maxSteps: plasma.maxSteps(),
    x: 0, y: i*segmh,
    w: plasma.width(), h: plasma.height()
  };
  opts.h = Math.min(segmh, plasma.height() - opts.y);
  const th = opts.h * TARGET_HEIGHT / opts.height >>> 0;
  const ty = opts.y * TARGET_HEIGHT / opts.height >>> 0;

  worker.onmessage = (e) => {
    var data = e.data;
    if (data === 'ready') {
      worker.postMessage(opts);
      requestNext(plasma, worker);
    }
    else if (data instanceof ImageBitmap) {
      sink.push(() => {
        requestNext(plasma, worker);
        let { w, h } = opts;
        for (let y = ty; y < canvas.height; y+= TARGET_HEIGHT) {
          for (let x = 0; x < canvas.width; x+= TARGET_WIDTH) {
            ctx.drawImage(data, 0, 0, w, h, x, y, TARGET_WIDTH, th);
          }
        }
        data.close();        
      });
      if (sink.length === numworkers) {
        plasma.exportedPA = null;
        plasma.update();
        requestAnimationFrame(ts => {
          sink.forEach(closure => closure());
          sink.length = 0;
          if (++frame_count === 100) {
            let fps = 100000 / (ts - last_ts);
            frame_count = 0;
            last_ts = ts;
            console.log("fps: ", fps);
          }
        });
      }
    }
    else {
      throw new Error("unknown message from worker");
    }
  };
});

function requestNext(plasma, worker) {
  let { buffer } = plasma.exportedPA || (plasma.exportedPA = plasma.exportPhaseAmps());
  buffer = buffer.slice();
  worker.postMessage(buffer, [buffer]);
}
