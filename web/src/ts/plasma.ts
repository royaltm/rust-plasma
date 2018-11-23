export { PlasmaHandle } from '../../plasma_web';
import { PlasmaHandle } from '../../plasma_web';

export interface PlasmaGeneratorOptions {
   minSteps?: number,
   maxSteps?: number
   width?: number,
   height?: number,
   parallel?: boolean,
   workerUrl?: string,
   workers?: number,
}

export interface BitmapDetail {
   // left offset in original bitmap
   x: number,
   // top offset in original bitmap
   y: number,
   // original plasma width
   width: number,
   // original plasma height
   height: number,
   // fragment of original bitmap
   bitmap: ImageBitmap
}

export const DefaultPlasmaGeneratorOptions: PlasmaGeneratorOptions = {
   minSteps: 80,
   maxSteps: 200,
   width: 128,
   height: 128,
   parallel: true,
   workerUrl: 'worker.js',
   workers: Math.min(2, Math.max(1, (self.navigator.hardwareConcurrency)))
}

export interface CanvasRendererOptions {
   width?: number,
   height?: number,
}

export const DefaultCanvasRendererOptions: CanvasRendererOptions = {
   width: 256,
   height: 256,
}

interface WorkerPlasmaSetup {
   width: number,
   height: number,
   minSteps: number,
   maxSteps: number,
   x: number,
   y: number,
   w: number,
   h: number
}

export class CanvasRenderer implements CanvasRendererOptions {
   /** target width */
   width: number;
   /** target height */
   height: number;
   /** is renderer animating */
   animating: boolean;
   /** target canvas */
   target: HTMLCanvasElement;
   /** bitmap generator */
   generator: BitmapGenerator;
   ctx: CanvasRenderingContext2D;
   protected readyHandler: () => void;
   protected bitmapHandler: (event: CustomEvent) => void;
   protected spool: BitmapDetail[];
   /**
    * Creates a new canvas renderer instance.
    */
   constructor(target: HTMLCanvasElement, options?: CanvasRendererOptions) {
      Object.assign(this, DefaultCanvasRendererOptions, options);
      this.target = target;
      this.ctx = target.getContext('2d');
      this.animating = false;
      this.spool = [];
      this.readyHandler = () => {
         let spool = this.spool;
         if (spool.length !== 0) this.spool= [];
         requestAnimationFrame(_ts => {
            let ctx = this.ctx;
            let canvas = this.target;
            let target_width = this.width, target_height = this.height;
            let canvas_width = canvas.width, canvas_height = canvas.height;
            spool.forEach(detail => {
               let { x, y, width, height, bitmap }: BitmapDetail = detail;
               let w = bitmap.width, h = bitmap.height;
               let dw = target_width / width, dh = target_height / height;
               let x0 = x * dw >>> 0, y0 = y * dh >>> 0;
               let tw = w * dw >>> 0, th = h * dh >>> 0;
               for (let ty = y0; ty < canvas_height; ty+= target_height) {
                  for (let tx = x0; tx < canvas_width; tx+= target_width) {
                     ctx.drawImage(bitmap, 0, 0, w, h, tx, ty, tw, th);
                  }
               }
               bitmap.close();
            });
            if (this.animating) {
               let generator = this.generator;
               if (generator) generator.render();
            }
         });
      };
      this.bitmapHandler = (ev) => {
         this.spool.push(ev.detail);
      };
   }

   /**
    * Attaches a bitmap generator instance to this renderer.
    */
   attach(generator: BitmapGenerator): void {
      this.animate(false);
      this.detach();
      this.generator = generator;
      generator.addEventListener("ready", this.readyHandler, false);
      generator.addEventListener("bitmap", this.bitmapHandler, false);
   }

   /**
    * Detaches a bitmap generator instance from this renderer.
    */
   detach(): BitmapGenerator {
      var generator = this.generator;
      if (generator) {
         this.animate(false);
         generator.removeEventListener("ready", this.readyHandler, false);
         generator.removeEventListener("bitmap", this.bitmapHandler, false);
      }
      this.generator = null;
      return generator;
   }

   /**
    * Controls animation state.
    */
   animate(enable: boolean): void {
      if (!enable === !this.animating) return;
      if (enable) {
         this.animating = true;
         let generator = this.generator;
         if (generator && generator.isReady) {
            generator.render();
         }
      }
      else {
         this.animating = false;
      }
   }
}

export interface BitmapGenerator extends EventTarget {
   readonly isReady: boolean;
   render(): boolean;
}

export class PlasmaGenerator extends EventTarget implements BitmapGenerator {
   protected handle: PlasmaHandle;
   protected workers: Worker[];
   protected queued: number;
   protected imageData: ImageData;
   /**
    * Creates a new generator instance.
    *
    * An event "ready" is being emitted as soon as the generator is ready for requests.
    */
   constructor(options?: PlasmaGeneratorOptions) {
      const opts: PlasmaGeneratorOptions = Object.assign({}, DefaultPlasmaGeneratorOptions, options);
      super();
      const plasma = this.handle = new PlasmaHandle(opts.width, opts.height, opts.minSteps, opts.maxSteps);
      this.workers = [];
      this.queued = -1;
      this.imageData = null;

      if (opts.parallel) {
         const [width, height] = [plasma.width(), plasma.height()];
         const workers = Math.max(1, Math.min(height, opts.workers));
         const segmentHeight = (height + workers - 1) / workers >>> 0;
         Array.from({length: workers}, () => new Worker(opts.workerUrl))
         .forEach((worker, index) => {
            const x: number = 0, y: number = index * segmentHeight;
            const setup: WorkerPlasmaSetup = {
               width,
               height,
               minSteps: plasma.minSteps(),
               maxSteps: plasma.maxSteps(),
               x, y,
               w: width,
               h: Math.min(segmentHeight, height - y)
            };
            worker.onmessage = (e) => {
               const data = e.data;
               if (data === 'ready') {
                  worker.postMessage(setup);
                  if (this.workers.push(worker) == workers) {
                     this.queued = 0;
                     this.dispatchEvent(new Event('ready'));
                  }
               }
               else if (data instanceof ImageBitmap) {
                  let detail: BitmapDetail = {x, y, width, height, bitmap: data};
                  this.dispatchEvent(new CustomEvent('bitmap', { detail }));
                  if (--this.queued === 0) {
                     plasma.update();
                     this.dispatchEvent(new Event('ready'));
                  }
               }
               else {
                  this.dispatchEvent(new CustomEvent('error', {
                     detail: new Error("Unknown message from worker")
                  }));
               }
            };
         });
      }
      else {
         setTimeout(() => {
            this.queued = 0;
            this.imageData = plasma.imageData();
            this.dispatchEvent(new Event('ready'));
         }, 1);
      }
   }

   /** Is true if ready for the next frame. */
   get isReady(): boolean {
      return this.queued === 0;
   }

   /**
    *  Request next bitmap rendition.
    *
    *  A "bitmap" CutomEvent (or many events) with a property detail
    *  of type BitmapDetail is being emitted.
    *
    *  An "error" Event is being emitted in case of an error.
    *
    *  A "ready" Event is being emitted when generator is again ready for a next request.
    *
    *  @returns true if a request was accepted.
    */
   render(): boolean {
      if (this.queued !== 0) return false;
      const workers = this.workers;
      let count = workers.length;
      const plasma: PlasmaHandle = this.handle;
      if (count > 0) {
         this.queued = count;
         let { buffer } = plasma.exportPhaseAmps();
         workers.forEach((worker, index) => {
            let buf = (index + 1 < count) ? buffer.slice(0) : buffer;
            worker.postMessage(buf, [buf]);
         });
      }
      else {
         this.queued = -1;
         plasma.render();
         createImageBitmap(this.imageData).then(bitmap => {
            let { width, height } = bitmap;
            let detail: BitmapDetail = {x: 0, y: 0, width, height, bitmap};
            this.dispatchEvent(new CustomEvent('bitmap', { detail }));
            plasma.update();
            this.queued = 0;
            this.dispatchEvent(new Event('ready'));
         }, error => {
            this.dispatchEvent(new CustomEvent('error', { detail: error }));
         });
      }
      return true;
   }
}
