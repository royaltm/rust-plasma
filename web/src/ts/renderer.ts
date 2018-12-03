/**
 * @module plasma
 */
/**/
import { BitmapGenerator } from './generator';

/**
 *  A base renderer class.
 *
 *  Attach an instance of a [[BitmapGenerator]] with a [[BaseRenderer.attach]] method.
 *
 *  Implementors must provide [[BaseRenderer.readyHandler]] and [[BaseRenderer.bitmapHandler]] as
 *  methods bound to this instance.
 */
export abstract class BaseRenderer {
    /** Animation state. */
    isAnimating: boolean;
    /** The attached bitmap generator. */
    generator: BitmapGenerator;
    protected readyHandler: () => void;
    protected bitmapHandler: (event: CustomEvent) => void;
    /**
     * Creates a new canvas renderer instance.
     */
    constructor() {
        this.isAnimating = false;
        this.generator = null;
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
