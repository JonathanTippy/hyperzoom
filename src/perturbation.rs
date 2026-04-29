use crate::*;
use crate::constants::*;
use crate::reference_orbit::*;
use std::ops::{Add, Sub};

#[inline(always)]
fn points_near<T: Add<Output = T> + Sub<Output = T> + PartialOrd + Clone>(z1: &(T, T), z2: &(T, T), epsilon: &T) -> bool {
    (z1.0 >= (z2.0.clone() - epsilon.clone())) && (z1.0 <= (z2.0.clone() + epsilon.clone()))
        && (z1.1 >= (z2.1.clone() - epsilon.clone())) && (z1.1 <= (z2.1.clone() + epsilon.clone()))
}

#[derive(Clone, Debug)]
pub(crate) struct PerturbationPoint<T> {
    pub delta_constant_complex: (T, T),
    pub delta_iterated_complex: (T, T),
    pub reference_index: usize,
    pub iterations: u64,
    pub loop_detection_point: ((T, T), u64),
    pub escapes: bool,
    pub repeats: bool,
    pub delivered: bool,
    pub period: u64,
    pub smallness_squared: T,
    pub iterations_to_smallest_distance: u64,
    pub escape_location_f32: (f32, f32),
}

impl<T: Mandelbrotable> PerturbationPoint<T> {
    pub(crate) fn new(delta_c: (T, T)) -> Self {
        let zero = T::from_f64(0.0);
        let max_dist = T::from_f64(f64::MAX);
        PerturbationPoint {
            delta_constant_complex: delta_c,
            delta_iterated_complex: (zero.clone(), zero.clone()),
            reference_index: 0,
            iterations: 0,
            loop_detection_point: ((zero.clone(), zero.clone()), 0),
            escapes: false,
            repeats: false,
            delivered: false,
            period: 0,
            smallness_squared: max_dist,
            iterations_to_smallest_distance: 0,
            escape_location_f32: (0.0, 0.0),
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        self.escapes || self.repeats
    }
}

#[inline]
fn iterate_perturbation_delta_on_point<T>(point: &mut PerturbationPoint<T>, reference_complex: &(f64, f64))
where
    T: Mandelbrotable,
{
    let reference_complex_as_type = (T::from_f64(reference_complex.0), T::from_f64(reference_complex.1));
    let two = T::from_f64(2.0);

    let two_times_reference_times_delta_real = (two.clone() * (reference_complex_as_type.0.clone() * &point.delta_iterated_complex.0)) - (two.clone() * (reference_complex_as_type.1.clone() * &point.delta_iterated_complex.1));
    let two_times_reference_times_delta_imag = (two.clone() * (reference_complex_as_type.0.clone() * &point.delta_iterated_complex.1)) + (two.clone() * (reference_complex_as_type.1.clone() * &point.delta_iterated_complex.0));

    let delta_squared_real = (point.delta_iterated_complex.0.clone() * &point.delta_iterated_complex.0) - (point.delta_iterated_complex.1.clone() * &point.delta_iterated_complex.1);
    let delta_squared_imag = two * (point.delta_iterated_complex.0.clone() * &point.delta_iterated_complex.1);

    let result_real = (two_times_reference_times_delta_real + &delta_squared_real) + &point.delta_constant_complex.0;
    let result_imag = (two_times_reference_times_delta_imag + &delta_squared_imag) + &point.delta_constant_complex.1;

    point.delta_iterated_complex = (result_real, result_imag);
}

#[inline]
pub(crate) fn check_glitch<T>(perturbed_complex: &(T, T), delta_complex: &(T, T)) -> bool
where
    T: Mandelbrotable,
{
    let perturbed_radius_squared = (perturbed_complex.0.clone() * &perturbed_complex.0) + (perturbed_complex.1.clone() * &perturbed_complex.1);
    let delta_radius_squared = (delta_complex.0.clone() * &delta_complex.0) + (delta_complex.1.clone() * &delta_complex.1);

    perturbed_radius_squared < delta_radius_squared
}

#[inline]
pub(crate) fn iterate_perturbation_point<T>(point: &mut PerturbationPoint<T>, reference_orbit: &ReferenceOrbit, epsilon: T)
where
    T: Mandelbrotable,
{
    if point.escapes || point.repeats {
        return;
    }

    let reference_complex = match reference_orbit.get_z(point.reference_index) {
        Some(z) => z,
        None => {
            return;
        }
    };

    if point.reference_index >= reference_orbit.len() {
        return;
    }

    let reference_complex_as_type = (T::from_f64(reference_complex.0), T::from_f64(reference_complex.1));

    let perturbed_real = reference_complex_as_type.0.clone() + &point.delta_iterated_complex.0;
    let perturbed_imag = reference_complex_as_type.1.clone() + &point.delta_iterated_complex.1;
    let perturbed_complex = (perturbed_real, perturbed_imag);

    let perturbed_radius_squared = (perturbed_complex.0.clone() * &perturbed_complex.0) + (perturbed_complex.1.clone() * &perturbed_complex.1);

    if perturbed_radius_squared.clone() > T::from_f64(BAILOUT_RADIUS_SQUARED) {
        point.escapes = true;
        point.escape_location_f32 = (perturbed_complex.0.to_f64() as f32, perturbed_complex.1.to_f64() as f32);
        return;
    }

    if check_glitch(&perturbed_complex, &point.delta_iterated_complex) || point.reference_index >= reference_orbit.len() {
        point.delta_iterated_complex = perturbed_complex;
        point.reference_index = 0;
        point.iterations += 1;
        return;
    }

    let new_r_squared = perturbed_radius_squared.to_f64();
    if new_r_squared < point.smallness_squared.clone().to_f64() {
        point.smallness_squared = T::from_f64(new_r_squared);
        point.iterations_to_smallest_distance = point.iterations;
    }

    iterate_perturbation_delta_on_point(point, reference_complex);
    point.reference_index += 1;
    point.iterations += 1;

    if point.iterations > 1 {
        let lp = &point.loop_detection_point.0;
        let near = (point.delta_iterated_complex.0 >= lp.0.clone() - epsilon.clone()) &&
                   (point.delta_iterated_complex.0 <= lp.0.clone() + epsilon.clone()) &&
                   (point.delta_iterated_complex.1 >= lp.1.clone() - epsilon.clone()) &&
                   (point.delta_iterated_complex.1 <= lp.1.clone() + epsilon.clone());
        if near {
            point.repeats = true;
            point.period = point.iterations - point.loop_detection_point.1;
            return;
        }
    }

    if point.iterations >= point.loop_detection_point.1 << 1 {
        point.loop_detection_point = (point.delta_iterated_complex.clone(), point.iterations);
    }
}


