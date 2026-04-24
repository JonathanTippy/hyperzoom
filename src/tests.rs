use crate::{Point, iterate_z, compute_r_squared, check_bailout};
use rug::Float;

// Helper function to create Float from f64 value
fn float_from(v: f64) -> Float {
    Float::with_val_64(1024, v)
}

// ============================================================================
// 10 KNOWN TEST POINTS FOR iterate_z (f32 and f64 only)
// Note: rug::Float cannot be used with iterate_z because Float does not
// implement Add<&Float, Output=Float> - it returns AddIncomplete<'_>.
// ============================================================================

// Test point 1: c = (0, 0) - origin stays at origin forever
#[test]
fn test_origin_stays_at_origin_f32() {
    let c = (0.0f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f32, 0.0f32));
}

#[test]
fn test_origin_stays_at_origin_f64() {
    let c = (0.0f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f64, 0.0f64));
}

// Test point 2: c = (-1, 0) - period 2 cycle starting from origin
#[test]
fn test_c_neg1_first_iteration_f32() {
    let c = (-1.0f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-1.0f32, 0.0f32));
}

#[test]
fn test_c_neg1_first_iteration_f64() {
    let c = (-1.0f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-1.0f64, 0.0f64));
}

// Test point 3: c = (0, 1) - imaginary unit
#[test]
fn test_c_imaginary_unit_first_iteration_f32() {
    let c = (0.0f32, 1.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f32, 1.0f32));
}

#[test]
fn test_c_imaginary_unit_first_iteration_f64() {
    let c = (0.0f64, 1.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f64, 1.0f64));
}

// Test point 4: c = (0.25, 0) - boundary of main cardioid
#[test]
fn test_c_quarter_boundary_first_iteration_f32() {
    let c = (0.25f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.25f32, 0.0f32));
}

#[test]
fn test_c_quarter_boundary_first_iteration_f64() {
    let c = (0.25f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.25f64, 0.0f64));
}

// Test point 5: c = (-0.75, 0) - in the set (period 2 cycle)
#[test]
fn test_c_neg_three_quarters_first_iteration_f32() {
    let c = (-0.75f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-0.75f32, 0.0f32));
}

#[test]
fn test_c_neg_three_quarters_first_iteration_f64() {
    let c = (-0.75f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-0.75f64, 0.0f64));
}

// Test point 6: c = (2, 0) - escapes very quickly
#[test]
fn test_c_two_escapes_first_iteration_f32() {
    let c = (2.0f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (2.0f32, 0.0f32));
}

#[test]
fn test_c_two_escapes_first_iteration_f64() {
    let c = (2.0f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (2.0f64, 0.0f64));
}

// Test point 7: c = (1, 0) - escapes quickly
#[test]
fn test_c_one_escapes_first_iteration_f32() {
    let c = (1.0f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (1.0f32, 0.0f32));
}

#[test]
fn test_c_one_escapes_first_iteration_f64() {
    let c = (1.0f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (1.0f64, 0.0f64));
}

// Test point 8: c = (-2, 0) - boundary point (Misiurewicz point)
#[test]
fn test_c_neg_two_boundary_first_iteration_f32() {
    let c = (-2.0f32, 0.0f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-2.0f32, 0.0f32));
}

#[test]
fn test_c_neg_two_boundary_first_iteration_f64() {
    let c = (-2.0f64, 0.0f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-2.0f64, 0.0f64));
}

// Test point 9: c = (0, 0.5) - in the main cardioid
#[test]
fn test_c_half_imaginary_first_iteration_f32() {
    let c = (0.0f32, 0.5f32);
    let z = (0.0f32, 0.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f32, 0.5f32));
}

#[test]
fn test_c_half_imaginary_first_iteration_f64() {
    let c = (0.0f64, 0.5f64);
    let z = (0.0f64, 0.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f64, 0.5f64));
}

// Test point 10: Non-trivial iteration - z=(1,1), c=(0,0)
// z_1 = (1+1i)^2 + 0 = (1-1, 2*1*1) = (0, 2)
#[test]
fn test_nontrivial_iteration_f32() {
    let c = (0.0f32, 0.0f32);
    let z = (1.0f32, 1.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f32, 2.0f32));
}

#[test]
fn test_nontrivial_iteration_f64() {
    let c = (0.0f64, 0.0f64);
    let z = (1.0f64, 1.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (0.0f64, 2.0f64));
}

// Test point 11: Non-trivial iteration - z=(2,3), c=(1,-1)
// z_1 = (2+3i)^2 + (1-1i) = (4-9 + 12i) + (1-1i) = -4 + 11i
#[test]
fn test_nontrivial_iteration_with_c_f32() {
    let c = (1.0f32, -1.0f32);
    let z = (2.0f32, 3.0f32);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-4.0f32, 11.0f32));
}

#[test]
fn test_nontrivial_iteration_with_c_f64() {
    let c = (1.0f64, -1.0f64);
    let z = (2.0f64, 3.0f64);
    let result = iterate_z(&z, &c);
    assert_eq!(result, (-4.0f64, 11.0f64));
}

// Test point 12: Second iteration from origin with c = (-1, 0)
// z_0 = (0, 0), z_1 = (-1, 0), z_2 = (-1)^2 + 0 + (-1, 0) = (1, 0) + (-1, 0) = (0, 0)
#[test]
fn test_second_iteration_c_neg1_f32() {
    let c = (-1.0f32, 0.0f32);
    let z1 = (0.0f32, 0.0f32);
    let step1 = iterate_z(&z1, &c);
    assert_eq!(step1, (-1.0f32, 0.0f32));
    let step2 = iterate_z(&step1, &c);
    assert_eq!(step2, (0.0f32, 0.0f32));
}

#[test]
fn test_second_iteration_c_neg1_f64() {
    let c = (-1.0f64, 0.0f64);
    let z1 = (0.0f64, 0.0f64);
    let step1 = iterate_z(&z1, &c);
    assert_eq!(step1, (-1.0f64, 0.0f64));
    let step2 = iterate_z(&step1, &c);
    assert_eq!(step2, (0.0f64, 0.0f64));
}

// ============================================================================
// 10 KNOWN TEST POINTS FOR compute_r_squared (f32 and f64 only)
// Note: rug::Float cannot be used with compute_r_squared because Float
// does not implement Add<&Float, Output=Float>.
// ============================================================================

#[test]
fn test_compute_r_squared_f32() {
    let point = Point {
        c: (1.0f32, 2.0f32),
        z: (3.0f32, 4.0f32),
        real_squared: 9.0f32,
        imag_squared: 16.0f32,
        real_imag: 12.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 25.0f32);
}

#[test]
fn test_compute_r_squared_f64() {
    let point = Point {
        c: (1.0f64, 2.0f64),
        z: (3.0f64, 4.0f64),
        real_squared: 9.0f64,
        imag_squared: 16.0f64,
        real_imag: 12.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 25.0f64);
}

#[test]
fn test_compute_r_squared_zero_f32() {
    let point = Point {
        c: (0.0f32, 0.0f32),
        z: (0.0f32, 0.0f32),
        real_squared: 0.0f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.0f32);
}

#[test]
fn test_compute_r_squared_zero_f64() {
    let point = Point {
        c: (0.0f64, 0.0f64),
        z: (0.0f64, 0.0f64),
        real_squared: 0.0f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.0f64);
}

#[test]
fn test_compute_r_squared_unit_imaginary_f32() {
    let point = Point {
        c: (0.0f32, 1.0f32),
        z: (0.0f32, 1.0f32),
        real_squared: 0.0f32,
        imag_squared: 1.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 1.0f32);
}

#[test]
fn test_compute_r_squared_unit_imaginary_f64() {
    let point = Point {
        c: (0.0f64, 1.0f64),
        z: (0.0f64, 1.0f64),
        real_squared: 0.0f64,
        imag_squared: 1.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 1.0f64);
}

#[test]
fn test_compute_r_squared_quarter_f32() {
    let point = Point {
        c: (0.25f32, 0.0f32),
        z: (0.25f32, 0.0f32),
        real_squared: 0.0625f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.0625f32);
}

#[test]
fn test_compute_r_squared_quarter_f64() {
    let point = Point {
        c: (0.25f64, 0.0f64),
        z: (0.25f64, 0.0f64),
        real_squared: 0.0625f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.0625f64);
}

#[test]
fn test_compute_r_squared_neg_three_quarters_f32() {
    let point = Point {
        c: (-0.75f32, 0.0f32),
        z: (-0.75f32, 0.0f32),
        real_squared: 0.5625f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.5625f32);
}

#[test]
fn test_compute_r_squared_neg_three_quarters_f64() {
    let point = Point {
        c: (-0.75f64, 0.0f64),
        z: (-0.75f64, 0.0f64),
        real_squared: 0.5625f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.5625f64);
}

#[test]
fn test_compute_r_squared_two_f32() {
    let point = Point {
        c: (2.0f32, 0.0f32),
        z: (2.0f32, 0.0f32),
        real_squared: 4.0f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 4.0f32);
}

#[test]
fn test_compute_r_squared_two_f64() {
    let point = Point {
        c: (2.0f64, 0.0f64),
        z: (2.0f64, 0.0f64),
        real_squared: 4.0f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 4.0f64);
}

#[test]
fn test_compute_r_squared_one_f32() {
    let point = Point {
        c: (1.0f32, 0.0f32),
        z: (1.0f32, 0.0f32),
        real_squared: 1.0f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 1.0f32);
}

#[test]
fn test_compute_r_squared_one_f64() {
    let point = Point {
        c: (1.0f64, 0.0f64),
        z: (1.0f64, 0.0f64),
        real_squared: 1.0f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 1.0f64);
}

#[test]
fn test_compute_r_squared_neg_two_f32() {
    let point = Point {
        c: (-2.0f32, 0.0f32),
        z: (-2.0f32, 0.0f32),
        real_squared: 4.0f32,
        imag_squared: 0.0f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 4.0f32);
}

#[test]
fn test_compute_r_squared_neg_two_f64() {
    let point = Point {
        c: (-2.0f64, 0.0f64),
        z: (-2.0f64, 0.0f64),
        real_squared: 4.0f64,
        imag_squared: 0.0f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 4.0f64);
}

#[test]
fn test_compute_r_squared_half_imaginary_f32() {
    let point = Point {
        c: (0.0f32, 0.5f32),
        z: (0.0f32, 0.5f32),
        real_squared: 0.0f32,
        imag_squared: 0.25f32,
        real_imag: 0.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.25f32);
}

#[test]
fn test_compute_r_squared_half_imaginary_f64() {
    let point = Point {
        c: (0.0f64, 0.5f64),
        z: (0.0f64, 0.5f64),
        real_squared: 0.0f64,
        imag_squared: 0.25f64,
        real_imag: 0.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 0.25f64);
}

#[test]
fn test_compute_r_squared_nontrivial_f32() {
    let point = Point {
        c: (0.0f32, 0.0f32),
        z: (1.0f32, 1.0f32),
        real_squared: 1.0f32,
        imag_squared: 1.0f32,
        real_imag: 1.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 2.0f32);
}

#[test]
fn test_compute_r_squared_nontrivial_f64() {
    let point = Point {
        c: (0.0f64, 0.0f64),
        z: (1.0f64, 1.0f64),
        real_squared: 1.0f64,
        imag_squared: 1.0f64,
        real_imag: 1.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 2.0f64);
}

#[test]
fn test_compute_r_squared_nontrivial_with_c_f32() {
    let point = Point {
        c: (1.0f32, -1.0f32),
        z: (-4.0f32, 11.0f32),
        real_squared: 16.0f32,
        imag_squared: 121.0f32,
        real_imag: -44.0f32,
        iterations: 0,
        loop_detection_point: ((0.0f32, 0.0f32), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f32,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 137.0f32);
}

#[test]
fn test_compute_r_squared_nontrivial_with_c_f64() {
    let point = Point {
        c: (1.0f64, -1.0f64),
        z: (-4.0f64, 11.0f64),
        real_squared: 16.0f64,
        imag_squared: 121.0f64,
        real_imag: -44.0f64,
        iterations: 0,
        loop_detection_point: ((0.0f64, 0.0f64), 0),
        escapes: false,
        repeats: false,
        delivered: false,
        period: 0,
        smallness_squared: 0.0f64,
        small_time: 0,
    };
    let r_sq = compute_r_squared(&point);
    assert_eq!(r_sq, 137.0f64);
}

// ============================================================================
// 10 KNOWN TEST POINTS FOR check_bailout (f32, f64, and rug::Float)
// ============================================================================

#[test]
fn test_check_bailout_true_f32() {
    let r_squared = 5.0f32;
    let threshold = 4.0f32;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_true_f64() {
    let r_squared = 5.0f64;
    let threshold = 4.0f64;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_true_rug() {
    assert!(check_bailout(float_from(5.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_false_f32() {
    let r_squared = 3.0f32;
    let threshold = 4.0f32;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_false_f64() {
    let r_squared = 3.0f64;
    let threshold = 4.0f64;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_false_rug() {
    assert!(!check_bailout(float_from(3.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_equal_f32() {
    let r_squared = 4.0f32;
    let threshold = 4.0f32;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_equal_f64() {
    let r_squared = 4.0f64;
    let threshold = 4.0f64;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_equal_rug() {
    assert!(!check_bailout(float_from(4.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_large_value_f32() {
    let r_squared = 1000000.0f32;
    let threshold = 4.0f32;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_large_value_f64() {
    let r_squared = 1000000.0f64;
    let threshold = 4.0f64;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_large_value_rug() {
    assert!(check_bailout(float_from(1000000.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_zero_f32() {
    let r_squared = 0.0f32;
    let threshold = 4.0f32;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_zero_f64() {
    let r_squared = 0.0f64;
    let threshold = 4.0f64;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_zero_rug() {
    assert!(!check_bailout(float_from(0.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_negative_f32() {
    let r_squared = -1.0f32;
    let threshold = 4.0f32;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_negative_f64() {
    let r_squared = -1.0f64;
    let threshold = 4.0f64;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_negative_rug() {
    assert!(!check_bailout(float_from(-1.0), float_from(4.0)));
}

#[test]
fn test_check_bailout_just_above_threshold_f32() {
    let r_squared = 4.0001f32;
    let threshold = 4.0f32;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_just_above_threshold_f64() {
    let r_squared = 4.0001f64;
    let threshold = 4.0f64;
    assert!(check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_just_above_threshold_rug() {
    assert!(check_bailout(float_from(4.0001), float_from(4.0)));
}

#[test]
fn test_check_bailout_just_below_threshold_f32() {
    let r_squared = 3.9999f32;
    let threshold = 4.0f32;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_just_below_threshold_f64() {
    let r_squared = 3.9999f64;
    let threshold = 4.0f64;
    assert!(!check_bailout(r_squared, threshold));
}

#[test]
fn test_check_bailout_just_below_threshold_rug() {
    assert!(!check_bailout(float_from(3.9999), float_from(4.0)));
}