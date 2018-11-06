use std::cmp::min;
use std::f32::consts::PI;

use rand::Rng;

use palette::{LinSrgb, Hsv, RgbHue, IntoColor};

use phase_amp::*;

use fast_math::{sin, cos};

const PI2: f32 = 2.0*PI;

type PhaseAmpsT = [PhaseAmp; 24];

#[derive(Debug, Clone, PartialEq)]
pub struct Plasma {
    pub pixel_width: u32,
    pub pixel_height: u32,
    config: PhaseAmpCfg,
    phase_amps: PhaseAmpsT,
}

pub trait PixelBuffer {
    fn put_pixel<C: IntoColor>(buffer: &mut [u8], offset: usize, pixel: C) -> usize;
}

pub struct PixelRGB24;

impl PixelBuffer for PixelRGB24 {
    #[inline(always)]
    fn put_pixel<C: IntoColor>(buffer: &mut [u8], offset: usize, pixel: C) -> usize {
        let rgb: LinSrgb = pixel.into_rgb();
        buffer[offset] = rgb.red.to_color8();
        buffer[offset + 1] = rgb.green.to_color8();
        buffer[offset + 2] = rgb.blue.to_color8();
        offset + 3
    }    
}

pub struct PixelRGBA8;

impl PixelBuffer for PixelRGBA8 {
    #[inline(always)]
    fn put_pixel<C: IntoColor>(buffer: &mut [u8], offset: usize, pixel: C) -> usize {
        let rgb: LinSrgb = pixel.into_rgb();
        buffer[offset] = rgb.red.to_color8();
        buffer[offset + 1] = rgb.green.to_color8();
        buffer[offset + 2] = rgb.blue.to_color8();
        buffer[offset + 3] = 255;
        offset + 4
    }    
}

impl Plasma {
    pub fn new<R: Rng + ?Sized>(pixel_width: u32, pixel_height: u32, config: PhaseAmpCfg, rng: &mut R) -> Self {
        let mut phase_amps = PhaseAmpsT::default();
        for p in phase_amps.iter_mut() {
            *p = PhaseAmp::new(&config, rng);
        }
        Plasma {
            pixel_width, pixel_height, config, phase_amps
        }
    }

    pub fn update<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let config = &self.config;
        for pa in self.phase_amps.iter_mut() {
            pa.update(config, rng);
        }
    }

    #[inline(always)]
    pub fn render<B: PixelBuffer>(&self, buffer: &mut [u8], pitch: usize) {
        self.render_part::<B>(buffer, pitch, 0, 0, self.pixel_width as usize, self.pixel_height as usize, 0)
    }

    /// offset is the current starting offset of the buffer slice provided
    #[inline(always)]
    pub fn render_part<B: PixelBuffer>(&self, buffer: &mut [u8], pitch: usize, x: usize, y: usize, w: usize, h: usize, offset: usize) {
        let pw = self.pixel_width as usize;
        let ph = self.pixel_height as usize;
        let phase_amps = &self.phase_amps[..];
        render_part::<B, _>(buffer, pitch, pw, ph, phase_amps, x, y, w, h, offset)
    }

    pub fn import_phase_amps(&mut self, data: &[f32]) {
        for (i, pa) in self.phase_amps.iter_mut().enumerate() {
            let src = data.at(i);
            pa.set_phase(src.phase());
            pa.set_amplitude(src.amplitude());
        }
    }

    pub fn export_phase_amps(&self) -> Vec<f32> {
        let mut out = Vec::new();
        self.phase_amps.export(&mut out);
        out
    }

    #[inline(always)]
    pub fn min_steps(&self) -> f32 {
        self.config.min_steps()
    }

    #[inline(always)]
    pub fn max_steps(&self) -> f32 {
        self.config.max_steps()
    }
}

pub fn render_part<B, P>(buffer: &mut [u8], pitch: usize, pw: usize, ph: usize, phase_amps: &P, x: usize, y: usize, w: usize, h: usize, offset: usize)
where B: PixelBuffer, P: PhaseAmpsSelect + ?Sized
{
    if x >= pw { return }
    else if y >= ph { return }
    let x2 = min(pw, x + w);
    let y2 = min(ph, y + h);
    let wr = pw as f32 / PI2;
    let hr = ph as f32 / PI2;
    for y in y..y2 {
        let mut offset = y * pitch - offset;
        let y = y as f32 / hr;
        for x in x..x2 {
            let x = x as f32 / wr;
            let pixel = gen_pixel(x, y, phase_amps.select(0..24));
            offset = B::put_pixel(buffer, offset, pixel);
        }
    }
}

#[inline(always)]
fn gen_pixel<P: PhaseAmpsSelect + ?Sized>(x: f32, y: f32, phase_amps: &P) -> LinSrgb {
    let hue0 = compose4(x, y, phase_amps.select(0..8));
    let hue1 = compose4(x, y, phase_amps.select(8..16));
    let sat0 = compose4(x, y, phase_amps.select(16..24)).to_val();
    // let rgb0 = LinSrgb::from(Hsv::new(hue0, 1.0, 1.0));
    // let rgb1 = LinSrgb::from(Hsv::new(hue1, sat, 1.0));
    // let val0 = compose4(x, y, phase_amps.select(4..12)).to_val();
    // let val1 = compose4(x, y, phase_amps.select(8..16)).to_val();
    // let val2 = compose4(x, y, phase_amps.select(16..24)).to_val();
    let rgb0 = LinSrgb::from(Hsv::new((1.0 - hue0*2.0).to_hue(), 1.0, 1.0));
    let rgb1 = LinSrgb::from(Hsv::new((hue1*3.0).to_hue(), sat0*3.0, 1.0));
    // let rgb = LinSrgb::from(Hsv::new(1.33.to_hue(), 1.0, sat0));
    rgb0 - rgb1
    // rgb
}

const THIRD: f32 = 1.0 / 3.0;

#[inline(always)]
fn compose4<P: PhaseAmpsSelect + ?Sized>(x: f32, y: f32, phase_amps: &P) -> f32 {
    (compose(x, phase_amps.at(0), phase_amps.at(1)) +
    (compose(y, phase_amps.at(2), phase_amps.at(3)) * compose(x, phase_amps.at(4), phase_amps.at(5))) +
    compose(y, phase_amps.at(6), phase_amps.at(7))) * THIRD
}

#[inline(always)]
fn compose<P: PhaseAmpAccess + ?Sized>(x: f32, pa1: &P, pa2: &P) -> f32 {
    let nor = pa1.amplitude() + pa2.amplitude();
    if nor == 0.0 {
        0.0
    }
    else {
        (
            sin(x + pa1.phase())*pa1.amplitude()
          + cos(x + pa2.phase())*pa2.amplitude()
        ) / nor
    }
}

trait ToColor {
    fn to_color8(&self) -> u8;
    fn to_hue(&self) -> RgbHue;
    fn to_val(&self) -> f32;
}

impl ToColor for f32 {
    #[inline(always)]
    fn to_color8(&self) -> u8 {
        match self.abs() * 255.0 {
            c if c > 255.0 => 255,
            c => c as u8
        }
    }

    #[inline(always)]
    fn to_hue(&self) -> RgbHue {
        RgbHue::from_degrees(self * 180.0)
    }

    #[inline(always)]
    fn to_val(&self) -> f32 {
        match self.abs() {
            v if v > 1.0 => 1.0,
            v => v
        }
    }
}
