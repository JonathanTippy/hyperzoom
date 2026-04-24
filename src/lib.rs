pub mod constants;
pub mod numtypes;
pub mod mandelbrot;
#[cfg(test)]
mod tests;

pub use numtypes::{IntExp, Mandelbrotable};
pub use mandelbrot::{Point, compute_r_squared, check_bailout, iterate_z, iterate_point};