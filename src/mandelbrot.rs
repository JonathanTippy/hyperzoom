use crate::*;
use crate::constants::*;
use crate::perturbation::PerturbationPoint;

#[derive(Clone, Debug)]
pub struct Point<T> {
    pub c: (T, T),
    pub z: (T, T),
    pub real_squared: T,
    pub imag_squared: T,
    pub real_imag: T,
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

#[derive(Clone, Debug, PartialEq)]
pub struct CompletedPoint {
    pub iterations_to_pass_bailout_radius: u64,
    pub escape_location_f32: (f32, f32),
    pub escapes: bool,
    pub converges: bool,
    pub period: u64,
    pub smallest_distance_squared: f64,
    pub iterations_to_smallest_distance: u64,
}

#[inline]
pub fn compute_r_squared<T>(point: &Point<T>) -> T
where
    T: Mandelbrotable,
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    &point.real_squared + &point.imag_squared
}

#[inline]
pub fn iterate_point<T: Mandelbrotable>(point: &mut Point<T>, epsilon: T)
where
    for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
    for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    let r_squared = (&point.z.0 * &point.z.0) + (&point.z.1 * &point.z.1);
    let bailout_radius_squared = T::from_f64(BAILOUT_RADIUS_SQUARED);

    if r_squared.clone() > bailout_radius_squared {
        point.escapes = true;
        point.escape_location_f32 = (point.z.0.clone().to_f64() as f32, point.z.1.clone().to_f64() as f32);
        return;
    }

    if point.iterations > 1 {
        let lp = &point.loop_detection_point.0;
        let near = (point.z.0 >= lp.0.clone() - epsilon.clone()) &&
                   (point.z.0 <= lp.0.clone() + epsilon.clone()) &&
                   (point.z.1 >= lp.1.clone() - epsilon.clone()) &&
                   (point.z.1 <= lp.1.clone() + epsilon.clone());
        if near {
            point.repeats = true;
            point.period = point.iterations - point.loop_detection_point.1;
            return;
        }
    }

    if point.iterations >= point.loop_detection_point.1 << 1 {
        point.loop_detection_point = (point.z.clone(), point.iterations);
    }

    let two = T::from_f64(2.0);
    let result_real = (&point.real_squared - &point.imag_squared) + &point.c.0;
    let result_imag = (&two * &point.real_imag) + &point.c.1;
    point.z = (result_real, result_imag);

    let z0 = &point.z.0;
    let z1 = &point.z.1;
    point.real_squared = z0 * z0;
    point.imag_squared = z1 * z1;
    point.real_imag = z0 * z1;

    let new_r_squared = (&point.real_squared + &point.imag_squared).to_f64();
    if point.iterations == 0 || new_r_squared < point.smallness_squared.clone().to_f64() {
        point.smallness_squared = T::from_f64(new_r_squared);
        point.iterations_to_smallest_distance = point.iterations;
    }

    point.iterations += 1;
}

impl CompletedPoint {
    pub fn from_point<T: Mandelbrotable>(point: &Point<T>) -> Self {
        CompletedPoint {
            iterations_to_pass_bailout_radius: point.iterations,
            escape_location_f32: point.escape_location_f32,
            escapes: point.escapes,
            converges: point.repeats,
            period: point.period,
            smallest_distance_squared: point.smallness_squared.clone().to_f64(),
            iterations_to_smallest_distance: point.iterations_to_smallest_distance,
        }
    }

    pub fn from_perturbation_point<T: Mandelbrotable>(point: &PerturbationPoint<T>) -> Self {
        CompletedPoint {
            iterations_to_pass_bailout_radius: point.iterations,
            escape_location_f32: point.escape_location_f32,
            escapes: point.escapes,
            converges: point.repeats,
            period: point.period,
            smallest_distance_squared: point.smallness_squared.clone().to_f64(),
            iterations_to_smallest_distance: point.iterations_to_smallest_distance,
        }
    }
}

impl<T: Mandelbrotable> Point<T> {
    pub fn new(c: (T, T)) -> Self {
        let zero = T::from_f64(0.0);
        let z = (zero.clone(), zero.clone());
        let rs = zero.clone();
        let is = zero.clone();
        let ri = zero.clone();
        let ss = T::from_f64(f64::MAX);

        Point {
            c,
            z,
            real_squared: rs,
            imag_squared: is,
            real_imag: ri,
            iterations: 0,
            loop_detection_point: ((T::from_f64(0.0), T::from_f64(0.0)), 0),
            escapes: false,
            repeats: false,
            delivered: false,
            period: 0,
            smallness_squared: ss,
            iterations_to_smallest_distance: 0,
            escape_location_f32: (0.0, 0.0),
        }
    }

    pub fn is_done(&self) -> bool {
        self.escapes || self.repeats
    }
}

