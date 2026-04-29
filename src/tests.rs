use crate::*;
use crate::constants::*;
use crate::reference_orbit::*;
use crate::perturbation::*;
use std::time::Duration;

fn test_point_naive_vs_perturbation_f64(c_real: f64, c_imag: f64, delta_real: f64, delta_imag: f64) {
    let epsilon = EPSILON_F64;
    
    let naive_c = (c_real + delta_real, c_imag + delta_imag);
    let mut naive_point = Point::<f64>::new(naive_c);
    while !naive_point.is_done() {
        iterate_point(&mut naive_point, epsilon.clone());
    }
    let naive_result = CompletedPoint::from_point(&naive_point);
    
    let reference_c = (c_real, c_imag);
    let mut reference_orbit = ReferenceOrbit::new(reference_c);
    let start = std::time::Instant::now();
    while !reference_orbit.is_complete() {
        if start.elapsed() > Duration::from_secs(10) {
            panic!("test timed out: reference orbit computation exceeded 10s");
        }
        reference_orbit.step_with_convergence_check();
    }
    
    let delta_c = (f64::from_f64(delta_real), f64::from_f64(delta_imag));
    let mut perturbation_point = PerturbationPoint::<f64>::new(delta_c);
    let start = std::time::Instant::now();
    while !perturbation_point.is_done() {
        if start.elapsed() > Duration::from_secs(10) {
            panic!("test timed out: perturbation computation exceeded 10s");
        }
        iterate_perturbation_point(&mut perturbation_point, &reference_orbit, epsilon.clone());
    }
    let perturbation_result = CompletedPoint::from_perturbation_point(&perturbation_point);
    
    assert_eq!(naive_result, perturbation_result, "Naive and perturbation results differ for point ({}, {}) + ({}, {})", c_real, c_imag, delta_real, delta_imag);
}

fn test_point_naive_vs_perturbation_f32(c_real: f32, c_imag: f32, delta_real: f32, delta_imag: f32) {
    let epsilon = EPSILON_F32;
    
    let naive_c = (c_real + delta_real, c_imag + delta_imag);
    let mut naive_point = Point::<f32>::new(naive_c);
    while !naive_point.is_done() {
        iterate_point(&mut naive_point, epsilon.clone());
    }
    let naive_result = CompletedPoint::from_point(&naive_point);
    
    let reference_c = (c_real as f64, c_imag as f64);
    let mut reference_orbit = ReferenceOrbit::new(reference_c);
    let start = std::time::Instant::now();
    while !reference_orbit.is_complete() {
        if start.elapsed() > Duration::from_secs(10) {
            panic!("test timed out: reference orbit computation exceeded 10s");
        }
        reference_orbit.step_with_convergence_check();
    }
    
    let delta_c = (delta_real, delta_imag);
    let mut perturbation_point = PerturbationPoint::<f32>::new(delta_c);
    let start = std::time::Instant::now();
    while !perturbation_point.is_done() {
        if start.elapsed() > Duration::from_secs(10) {
            panic!("test timed out: perturbation computation exceeded 10s");
        }
        iterate_perturbation_point(&mut perturbation_point, &reference_orbit, epsilon.clone());
    }
    let perturbation_result = CompletedPoint::from_perturbation_point(&perturbation_point);
    
    assert_eq!(naive_result, perturbation_result, "Naive and perturbation results differ for point ({}, {}) + ({}, {})", c_real, c_imag, delta_real, delta_imag);
}

#[test]
fn test_escaped_point_f64() {
    test_point_naive_vs_perturbation_f64(0.3, 0.3, 0.1, 0.1);
}

#[test]
fn test_convergent_point_f64() {
    test_point_naive_vs_perturbation_f64(0.3, 0.3, -0.05, -0.05);
}

#[test]
fn test_escaped_point_f32() {
    test_point_naive_vs_perturbation_f32(0.3, 0.3, 0.1, 0.1);
}

#[test]
fn test_convergent_point_f32() {
    test_point_naive_vs_perturbation_f32(0.3, 0.3, -0.05, -0.05);
}

#[test]
fn test_near_boundary_f64() {
    test_point_naive_vs_perturbation_f64(0.25, 0.25, 0.01, 0.01);
}

#[test]
fn test_near_boundary_f32() {
    test_point_naive_vs_perturbation_f32(0.25, 0.25, 0.01, 0.01);
}

#[test]
fn test_cardioid_interior_f64() {
    test_point_naive_vs_perturbation_f64(0.2, 0.2, -0.05, -0.05);
}

#[test]
fn test_cardioid_interior_f32() {
    test_point_naive_vs_perturbation_f32(0.2, 0.2, -0.05, -0.05);
}

#[test]
fn test_period_2_bulb_f64() {
    test_point_naive_vs_perturbation_f64(0.2, 0.2, -0.1, -0.1);
}

#[test]
fn test_period_2_bulb_f32() {
    test_point_naive_vs_perturbation_f32(0.2, 0.2, -0.1, -0.1);
}

#[test]
fn test_small_delta_f64() {
    test_point_naive_vs_perturbation_f64(0.3, 0.3, 0.001, 0.001);
}

#[test]
fn test_small_delta_f32() {
    test_point_naive_vs_perturbation_f32(0.3, 0.3, 0.001, 0.001);
}

#[test]
fn test_negative_coordinates_f64() {
    test_point_naive_vs_perturbation_f64(0.2, 0.2, -0.05, -0.05);
}

#[test]
fn test_negative_coordinates_f32() {
    test_point_naive_vs_perturbation_f32(0.2, 0.2, -0.05, -0.05);
}