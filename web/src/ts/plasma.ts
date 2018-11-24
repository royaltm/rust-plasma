/**
 * @module plasma
 */
export { PlasmaHandle } from '../../plasma_web';
import { PlasmaHandle } from '../../plasma_web';

/** Options passed to a [[PlasmaGenerator]] constructor. */
export interface PlasmaGeneratorOptions {
   /**
    *  A least possible count of radnom animation steps between phase and amplitude transitions.
    *
    *  The larger the number the slower plasma animates.
    */
   minSteps?: number,
   /**
    *  A largest possible count of radnom animation steps between phase and amplitude transitions.
    *
    *  The larger the number the slower plasma animates.
    */
   maxSteps?: number
   /** Width in pixels of the generated plasma. */
   width?: number,
   /** Height in pixels of the generated plasma. */
   height?: number,
   /** Should the rendering be performed with asynchronous worker threads. */
   parallel?: boolean,
   /** The worker URL. */
   workerUrl?: string,
   /** Specifies how many workers will be used for parallel rendition. */
   workers?: number,
}

/** Received in a `detail` property of a "bitmap" CustomEvent. */
export interface BitmapDetail {
   /** Left pixel offset of this fragment in the original bitmap. */
   x: number,
   /** Top pixel offset of this fragment in the original bitmap. */
   y: number,
   /** Original bitmap width. */
   width: number,
   /** Original bitmap height. */
   height: number,
   /** A fragment of the original bitmap. */
   bitmap: ImageBitmap
}

export const DefaultPlasmaGeneratorOptions: PlasmaGeneratorOptions = {
   minSteps: 80,
   maxSteps: 200,
   width: 128,
   height: 128,
   parallel: true,
   workerUrl: 'worker.js',
   workers: Math.min(2, Math.max(1, (self.navigator.hardwareConcurrency|0)))
}

/** Options passed to a [[CanvasRenderer]] constructor. */
export interface CanvasRendererOptions {
   /** Target pixel width of the rendered image. */
   width?: number,
   /** Target pixel height of the rendered image. */
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

/**
 * A renderer for a HTMLCanvasElement.
 *
 * Attach an instance of a [[BitmapGenerator]] with a [[CanvasRenderer.attach]] method.
 */
export class CanvasRenderer implements CanvasRendererOptions {
   /** Target image width in pixels. */
   width: number;
   /** Target image height in pixels. */
   height: number;
   /** Animation state. */
   isAnimating: boolean;
   /** The target canvas. */
   target: HTMLCanvasElement;
   /** The attached bitmap generator. */
   generator: BitmapGenerator;
   /** The rendering context of a target canvas. */
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
      this.isAnimating = false;
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
            if (this.isAnimating) {
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
      if (!enable === !this.isAnimating) return;
      if (enable) {
         this.isAnimating = true;
         let generator = this.generator;
         if (generator && generator.isReady) {
            generator.render();
         }
      }
      else {
         this.isAnimating = false;
      }
   }
}

/**
 *  An interface for a bitmap generator.
 *
 *  Add an event listener to listen to a "ready" and a "bitmap" events.
 * 
 *  May be attached to a [[CanvasRenderer]].
 */
export interface BitmapGenerator extends EventTarget {
   /** Readiness of the generator to render the next frame. */
   readonly isReady: boolean;
   /**
    *  Requests the next bitmap rendition.
    *
    *  A "bitmap" custom event with a property `detail`
    *  of the [[BitmapDetail]] type is being emitted.
    *
    *  There may be more than one event per single rendition
    *  if the data comes from multiple workers.
    *
    *  An "error" event is being emitted in case of an error.
    *
    *  A "ready" event is being emitted when generator finishes rendering
    *  the whole bitmap and can render again.
    *
    *  Returns `false` if the generator has not yet finished its previous rendition.
    *  In this instance wait for a "ready" event and try again once it arrives.
    */
   render(): boolean;
}

/**
 *  A plasma generator.
 */
export class PlasmaGenerator extends EventTarget implements BitmapGenerator {
   protected handle: PlasmaHandle;
   protected workers: Worker[];
   protected queued: number;
   protected imageData: ImageData;
   /**
    * Creates a new generator instance.
    *
    * An event "ready" is being emitted as soon as the generator is ready for work.
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

   get isReady(): boolean {
      return this.queued === 0;
   }

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
