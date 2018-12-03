/**
 * @module plasma
 */
/**/
import { BitmapDetail } from './generator';
import { BaseRenderer } from './renderer';

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

/**
 * A renderer for a HTMLCanvasElement.
 *
 * Attach an instance of a [[BitmapGenerator]] with a [[CanvasRenderer.attach]] method.
 */
export class CanvasRenderer extends BaseRenderer implements CanvasRendererOptions {
    /** Target image width in pixels. */
    width: number;
    /** Target image height in pixels. */
    height: number;
    /** The target canvas. */
    target: HTMLCanvasElement;
    /** The rendering context of a target canvas. */
    ctx: CanvasRenderingContext2D;
    protected spool: BitmapDetail[];
    /**
     * Creates a new canvas renderer instance.
     */
    constructor(target: HTMLCanvasElement, options?: CanvasRendererOptions) {
        super();
        Object.assign(this, DefaultCanvasRendererOptions, options);
        this.target = target;
        this.ctx = target.getContext('2d');
        this.spool = [];

        this.readyHandler = () => {
            var spool = this.spool;
            if (spool.length !== 0) this.spool= [];
            requestAnimationFrame(_ts => {
                if (!this.isAnimating) {
                    spool.forEach(({ bitmap }) => bitmap.close());
                    return;
                }
                var { ctx, target: canvas } = this
                , { width: target_width, height: target_height } = this
                , { width: canvas_width, height: canvas_height } = canvas
                spool.forEach(({ x, y, width, height, bitmap }) => {
                    var { width: w, height: h } = bitmap
                    , dw = target_width / width
                    , dh = target_height / height
                    , tw = w * dw >>> 0
                    , th = h * dh >>> 0
                    for (var ty = y * dh >>> 0;
                        ty < canvas_height;
                        ty+= target_height) {
                        for (var tx = x * dw >>> 0;
                            tx < canvas_width;
                            tx+= target_width) {
                            ctx.drawImage(bitmap, 0, 0, w, h, tx, ty, tw, th);
                    }
                }
                bitmap.close();
            });
                var generator = this.generator;
                if (generator) generator.render();
            });
        };

        this.bitmapHandler = (ev) => {
            this.spool.push(ev.detail);
        };
    }
}
