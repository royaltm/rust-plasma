use std::ops::Deref;
use std::f32::consts::PI;

lazy_static! {
    static ref SINTAB: SinTab = SinTab::new();
}

const PI2: f32 = 2.0*PI;
const PI05: f32 = 0.5*PI;
const SINTAB_LENGTH: usize = 4096;

struct SinTab([f32; SINTAB_LENGTH]);


impl Deref for SinTab {
    type Target = [f32; SINTAB_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SinTab {
    pub fn new() -> SinTab {
        let mut sintab = [0f32; SINTAB_LENGTH];
        for (i, p) in sintab.iter_mut().enumerate() {
            *p = (PI2 * i as f32 / SINTAB_LENGTH as f32).sin();
        }
        SinTab(sintab)
    }

    pub fn sin(&self, v: f32) -> f32 {
        if v < 0.0 {
            -self[(-v * SINTAB_LENGTH as f32 / PI2) as usize % SINTAB_LENGTH]
        }
        else {
            self[(v * SINTAB_LENGTH as f32 / PI2) as usize % SINTAB_LENGTH]
        }
    }
}

pub fn sin(v: f32) -> f32 {
    SINTAB.sin(v)
}

pub fn cos(v: f32) -> f32 {
    SINTAB.sin(v - PI05)
}
