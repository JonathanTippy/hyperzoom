use crate::Mandelbrotable;

#[derive(Clone, Debug)]
pub struct Point<T> {
    pub c: (T, T),
    pub z: (T, T),
    pub real_squared: T,
    pub imag_squared: T,
    pub real_imag: T,
    pub iterations: u32,
    pub loop_detection_point: ((T, T), u32),
    pub escapes: bool,
    pub repeats: bool,
    pub delivered: bool,
    pub period: u32,
    pub smallness_squared: T,
    pub small_time: u32,
}

pub fn compute_r_squared<T>(point: &Point<T>) -> T 
where 
    T: Mandelbrotable,
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    // Use borrowed-borrowed operation: &T + &T -> T (no clones)
    &point.real_squared + &point.imag_squared
}

pub fn check_bailout<T>(r_squared: T, threshold: T) -> bool 
where
    T: Mandelbrotable + PartialOrd
{
    r_squared > threshold
}

pub fn iterate_z<T: Mandelbrotable>(z: &(T, T), c: &(T, T)) -> (T, T)
where
    for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
    for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    // Use borrowed-borrowed operations: &T * &T -> T (no clones)
    let real_squared = &z.0 * &z.0;
    let imag_squared = &z.1 * &z.1;
    let real_imag = &z.0 * &z.1;
    
    // Use borrowed operations throughout: &T - &T + &T -> T, etc. (no clones)
    let two = T::from_f64(2.0);
    let result_real = (&real_squared - &imag_squared) + &c.0;
    let result_imag = (&two * &real_imag) + &c.1;
    (result_real, result_imag)
}

pub fn iterate_point<T: Mandelbrotable>(point: &mut Point<T>, bailout_threshold: T)
where
    for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
    for<'b> &'b T: std::ops::Add<&'b T, Output = T>,
    for<'c> &'c T: std::ops::Sub<&'c T, Output = T>,
{
    let r_squared = compute_r_squared(point);
    
    if check_bailout(r_squared, bailout_threshold) {
        point.escapes = true;
        return;
    }
    
    // Pass references to iterate_z (no clones needed)
    point.z = iterate_z(&point.z, &point.c);
    
    // Use borrowed-borrowed operations for cached values: &T * &T -> T (no clones)
    let z0 = &point.z.0;
    let z1 = &point.z.1;
    point.real_squared = z0 * z0;
    point.imag_squared = z1 * z1;
    point.real_imag = z0 * z1;
    
    point.iterations += 1;
}
