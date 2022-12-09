/**
 * @module plasma
 */
 /**/
import * as THREE from "three";
import { BitmapDetail } from './generator';
import { BaseRenderer } from './renderer';
/**
 * A renderer for a {@link three!Texture | THREE.Texture}.
 *
 * Attach an instance of a {@link BitmapGenerator} with a {@link TextureRenderer.attach} method.
 */
export class TextureRenderer extends BaseRenderer {
    /** The target Texture. */
    target: THREE.Texture;
    protected bitmap: ImageBitmap;
    protected canvas: OffscreenCanvas;
    protected ctx: OffscreenCanvasRenderingContext2D;
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
                let { canvas, ctx } = this;
                if (canvas == null || canvas.width !== width || canvas.height !== height) {
                    canvas = this.canvas = new OffscreenCanvas(width, height);
                    ctx = this.ctx = canvas.getContext("2d");// as CanvasRenderingContext2D;
                }
                ctx.drawImage(bitmap, x, y);
                bitmap.close();
            }
        };
    }
}
