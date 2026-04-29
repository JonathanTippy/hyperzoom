use crate::numtypes::*;
use crate::constants::*;

pub type Float = rug::Float;

#[derive(Clone, Debug)]
pub(crate) struct ReferenceOrbit {
    pub c: (Float, Float),
    pub z_sequence: Vec<(f64, f64)>,
    current_z: (Float, Float),
    loop_detection_z: (f64, f64),
    loop_detection_iter: u64,
    converged: bool,
}

impl ReferenceOrbit {
    pub(crate) fn new(c: (f64, f64)) -> Self {
        let c_float = (Float::from_f64(c.0), Float::from_f64(c.1));
        let current_z = (Float::from_f64(0.0), Float::from_f64(0.0));

        ReferenceOrbit {
            c: c_float,
            z_sequence: {
                let mut v = Vec::with_capacity(100_000);
                v.push((0.0f64, 0.0f64));
                v
            },
            current_z,
            loop_detection_z: (0.0f64, 0.0f64),
            loop_detection_iter: 0,
            converged: false,
        }
    }

    pub(crate) fn step(&mut self) {
        let z_real_squared = self.current_z.0.clone() * &self.current_z.0;
        let z_imag_squared = self.current_z.1.clone() * &self.current_z.1;
        let z_real_imag_product = self.current_z.0.clone() * &self.current_z.1;

        let new_z_real = (z_real_squared - z_imag_squared) + &self.c.0;
        let two = Float::from_f64(2.0);
        let new_z_imag = (two * z_real_imag_product) + &self.c.1;

        self.current_z = (new_z_real, new_z_imag);

        let z_as_f64 = (self.current_z.0.clone().to_f64(), self.current_z.1.clone().to_f64());

        self.z_sequence.push(z_as_f64);
    }

    pub(crate) fn is_complete(&self) -> bool {
        if self.z_sequence.is_empty() {
            return false;
        }

        let entry = self.z_sequence.last().unwrap();
        let r_squared = entry.0 * entry.0 + entry.1 * entry.1;
        if r_squared > BAILOUT_RADIUS_SQUARED as f64 {
            return true;
        }

        if self.converged {
            return true;
        }

        false
    }

    pub(crate) fn step_with_convergence_check(&mut self) {
        self.step();

        let z_as_f64 = (self.current_z.0.clone().to_f64(), self.current_z.1.clone().to_f64());
        let iter = (self.z_sequence.len() - 1) as u64;

        if iter > 0 {
            let dist = (z_as_f64.0 - self.loop_detection_z.0).abs() + (z_as_f64.1 - self.loop_detection_z.1).abs();
            if dist < EPSILON_F64 {
                self.converged = true;
                return;
            }
        }

        if iter >= self.loop_detection_iter << 1 {
            self.loop_detection_z = z_as_f64;
            self.loop_detection_iter = iter;
        }
    }
    
    pub(crate) fn get_z(&self, index: usize) -> Option<&(f64, f64)> {
        self.z_sequence.get(index)
    }
    
    pub(crate) fn get_z_times_2(&self, index: usize) -> Option<(f64, f64)> {
        self.z_sequence.get(index).map(|z| (z.0 * 2.0, z.1 * 2.0))
    }
    
    pub(crate) fn len(&self) -> usize {
        self.z_sequence.len()
    }
    
    pub(crate) fn is_empty(&self) -> bool {
        self.z_sequence.is_empty()
    }
}
