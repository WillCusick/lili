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
