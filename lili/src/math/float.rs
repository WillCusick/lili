#[cfg(not(f64))]
pub type Float = f32;

#[cfg(f64)]
pub type Float = f64;

pub const PI: Float = 3.14159265358979323846;
pub const INV_PI: Float = 0.31830988618379067154;
pub const INV_2PI: Float = 0.15915494309189533577;
pub const INV_4PI: Float = 0.07957747154594766788;
pub const PI_OVER_2: Float = 1.57079632679489661923;
pub const PI_OVER_4: Float = 0.78539816339744830961;

#[inline(always)]
pub fn radians(deg: Float) -> Float {
    (PI / 180.0) * deg
}

#[inline(always)]
pub fn degrees(rad: Float) -> Float {
    (180.0 / PI) * rad
}

pub fn smooth_step(x: Float, a: Float, b: Float) -> Float {
    if a == b {
        if x < a {
            return 0.0;
        }
        return 1.0;
    }

    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

#[inline(always)]
pub fn lerp(t: Float, a: Float, b: Float) -> Float {
    a * (1.0 - t) + b * t
}

#[inline(always)]
pub fn safe_sqrt(x: Float) -> Float {
    x.max(0.0).sqrt()
}

#[inline(always)]
pub fn sin_x_over_x(x: Float) -> Float {
    if 1.0 - x * x == 1.0 {
        return 1.0;
    }

    x.sin() / x
}

#[inline(always)]
pub fn safe_asin(x: Float) -> Float {
    x.clamp(-1.0, 1.0).asin()
}

#[inline(always)]
pub fn safe_acos(x: Float) -> Float {
    x.clamp(-1.0, 1.0).acos()
}

pub fn next_float_up(mut v: Float) -> Float {
    if v.is_infinite() && v > 0.0 {
        return v;
    }

    if v == -0.0 {
        v = 0.0;
    }

    let mut bits = v.to_bits();
    if v >= 0.0 {
        bits += 1;
    } else {
        bits -= 1;
    }

    Float::from_bits(bits)
}

pub fn next_float_down(mut v: Float) -> Float {
    if v.is_infinite() && v < 0.0 {
        return v;
    }

    if v == 0.0 {
        v = -0.0;
    }

    let mut bits = v.to_bits();
    if v > 0.0 {
        bits -= 1;
    } else {
        bits += 1;
    }

    Float::from_bits(bits)
}

#[cfg(not(f64))]
/// The largest representable value less than 1.0
/// f32::from_bits in const context is not yet stable (https://github.com/rust-lang/rust/issues/72447)
/// However, transmute is stable in const context, so we use that instead
pub const ONE_MINUS_EPSILON: Float = unsafe { std::mem::transmute(0x3f7fffffu32) };

#[cfg(f64)]
/// The largest representable value less than 1.0
/// f32::from_bits in const context is not yet stable (https://github.com/rust-lang/rust/issues/72447)
/// However, transmute is stable in const context, so we use that instead
pub const ONE_MINUS_EPSILON: Float = unsafe { std::mem::transmute(0x3fefffffffffffffu64) };
