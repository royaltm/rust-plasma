/**
 * @module plasma
 */
/**/
import { BitmapDetail } from './generator';
import { BaseRenderer } from './renderer';

// missing from typescript dom declarations
declare class OffscreenCanvas {
   height: number;
   width: number;
   constructor(width: number, height: number);
   getContext(contextType: "2d"|"bitmaprenderer", contextAttributes?: {alpha: boolean}): CanvasRenderingContext2D|ImageBitmapRenderingContext;
   toBlob(type: string, encoderOptions?: number): Promise<Blob>;
   transferToImageBitmap(): ImageBitmap;
}

/**
 * A renderer for a [[THREE.Texture]].
 *
 * Attach an instance of a [[BitmapGenerator]] with a [[TextureRenderer.attach]] method.
 */
export class TextureRenderer extends BaseRenderer {
   /** The target Texture. */
   target: THREE.Texture;
   protected bitmap: ImageBitmap;
   protected canvas: OffscreenCanvas;
   protected ctx: CanvasRenderingContext2D;
   /**
    * Creates a new Texture renderer instance.
    */
   constructor(target: THREE.Texture) {
      super();
      this.target = target;
      this.bitmap = null;
      this.canvas = null;
      this.ctx = null;

      this.readyHandler = () => {
         requestAnimationFrame(_ts => {
            var { bitmap, canvas, target: texture } = this
              , image = texture.image

            if (!this.isAnimating) {
               if (bitmap !== null) {
                  bitmap.close();
                  this.bitmap = null;
               }
               return;
            }
            if (bitmap !== null || canvas !== null) {
               if (bitmap !== null) {
                  this.bitmap = null;
                  texture.image = bitmap;
               }
               else {
                  texture.image = canvas.transferToImageBitmap();
               }
               texture.needsUpdate = true;
               if (image instanceof ImageBitmap) image.close();
            }
            var generator = this.generator;
            if (generator) generator.render();
         });
      };

      this.bitmapHandler = (ev) => {
         var { x, y, width, height, bitmap }: BitmapDetail = ev.detail;

         if (width === bitmap.width && height === bitmap.height) {
            this.bitmap = bitmap;
         }
         else {
            var { canvas, ctx } = this;
            if (canvas == null || canvas.width !== width || canvas.height !== height) {
               canvas = this.canvas = new OffscreenCanvas(width, height);
               ctx = this.ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
            }
            ctx.drawImage(bitmap, x, y);
            bitmap.close();
         }
      };
   }
}
