#![allow(unused_imports)]
use plasma::*;
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::closure::Closure;
use wasm_bindgen::{Clamped, JsCast};
// use web_sys::*;
use web_sys::{Window, ImageData, WorkerGlobalScope};
// use js_sys::JsCast;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // fn alert(s: &str);
}

struct Area {
    x: usize, y: usize, w: usize, h: usize
}

#[wasm_bindgen]
pub struct PlasmaHandle {
    plasma: Plasma,
    rng: OsRng,
    data: Vec<u8>,
    area: Area,
    wrkspc: Vec<u8>
}

#[wasm_bindgen]
impl PlasmaHandle {
    #[wasm_bindgen(constructor)]
    pub fn constructor(width: u32, height: u32, min_steps: f32, max_steps: f32) -> Result<PlasmaHandle, JsValue> {
        if min_steps <= 1.0 {
            return Err(js_sys::Error::new("steps should be larger than 1").into());
        }
        if max_steps <= min_steps {
            return Err(js_sys::Error::new("max steps should be larger than min steps").into());
        }
        let mut rng = OsRng::new().map_err(|e| js_sys::Error::new(e.msg))?;
        let cfg = PhaseAmpCfg::new(min_steps, max_steps);
        let plasma = Plasma::new(width, height, cfg, &mut rng);
        let data = vec![0; width as usize * height as usize * PixelRGBA8::PIXEL_BYTES];
        let wrkspc = Vec::new();
        Ok(PlasmaHandle {
            plasma,
            rng,
            data,
            area: Area { x: 0, y: 0, w: width as usize, h: height as usize },
            wrkspc
        })
    }

    #[wasm_bindgen(js_name=setArea)]
    pub fn set_area(&mut self, x: usize, y: usize, w: usize, h: usize) {
        if x + w > self.plasma.pixel_width as usize || y + h > self.plasma.pixel_height as usize {
            panic!("invalid area provided");
        }
        self.area = Area { x, y, w, h };
        self.data.resize(w * h * PixelRGBA8::PIXEL_BYTES, 0u8);
    }

    pub fn width(&self) -> u32 {
        self.plasma.pixel_width
    }

    pub fn height(&self) -> u32 {
        self.plasma.pixel_height
    }

    pub fn render(&mut self) {
        let Area { x, y, w, h } = self.area;
        let pitch: usize = PixelRGBA8::PIXEL_BYTES * w;
        self.plasma.render_part::<PixelRGBA8>(&mut self.data, pitch, x, y, w, h, Some(&mut self.wrkspc));
    }

    #[wasm_bindgen(js_name=renderPhaseAmps)]
    pub fn render_phase_amps(&mut self, phase_amps: &[f32]) {
        let Area { x, y, w, h } = self.area;
        let pitch: usize = PixelRGBA8::PIXEL_BYTES * w;
        let pw = self.plasma.pixel_width as usize;
        let ph = self.plasma.pixel_height as usize;
        let mixer = PlasmaMixer::new();
        render_part::<PixelRGBA8, PlasmaLineCalcProducer<_, _>, _, _>(&mixer, &mut self.data, pitch, pw, ph, phase_amps, x, y, w, h, Some(&mut self.wrkspc))
    }

    pub fn update(&mut self) {
        self.plasma.update(&mut self.rng);
    }

    #[wasm_bindgen(js_name=imageData)]
    pub fn image_data(&mut self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array(Clamped(&mut self.data), self.area.w as u32)
    }

    #[wasm_bindgen(js_name=createImageBitmap)]
    pub fn create_image_bitmap(&mut self) -> Result<js_sys::Promise, JsValue> {
        let image_data = self.image_data()?;
        let scope = self_()?;
        scope.create_image_bitmap_with_image_data(&image_data)
    }

    #[wasm_bindgen(js_name=exportPhaseAmps)]
    pub fn export_phase_amps(&self) -> Box<[f32]> {
        let mut out = Vec::new();
        self.plasma.export_phase_amps(&mut out);
        out.into_boxed_slice()
    }

    #[wasm_bindgen(js_name=importPhaseAmps)]
    pub fn import_phase_amps(&mut self, phase_amps: &[f32]) {
        self.plasma.import_phase_amps(phase_amps);
    }

    #[wasm_bindgen(js_name=minSteps)]
    pub fn min_steps(&self) -> f32 {
        self.plasma.min_steps()
    }

    #[wasm_bindgen(js_name=maxSteps)]
    pub fn max_steps(&self) -> f32 {
        self.plasma.max_steps()
    }
}


enum GlobalProxy {
    Window(Window),
    WorkerGlobalScope(WorkerGlobalScope),
    //... more scopes
}

impl GlobalProxy {
    fn create_image_bitmap_with_image_data(&self, a_image: &ImageData) -> Result<js_sys::Promise, JsValue> {
        match self {
            GlobalProxy::Window(window) => window.create_image_bitmap_with_image_data(a_image),
            GlobalProxy::WorkerGlobalScope(scope) => scope.create_image_bitmap_with_image_data(a_image),
            //... more of that
        }
    }
}

fn self_() -> Result<GlobalProxy, JsValue> {
    let global = js_sys::global();
    if js_sys::eval("typeof WorkerGlobalScope !== 'undefined'")?.as_bool().unwrap() {
        Ok(global.dyn_into::<WorkerGlobalScope>().map(GlobalProxy::WorkerGlobalScope)?)
    }
    else {
        Ok(global.dyn_into::<Window>().map(GlobalProxy::Window)?)
    }
}
