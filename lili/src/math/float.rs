/// The type used for floating-point numbers. It is either `f32` or `f64` depending on the configuration.
#[cfg(not(f64))]
pub type Float = f32;

#[cfg(f64)]
pub type Float = f64;

/// Extension trait for floating-point numbers.
pub trait FloatExt {
    /// Calculates the square root of the number, ensuring that the result is non-negative.
    fn safe_sqrt(self) -> Self;

    /// Calculates the arcsine of the number, ensuring that the result is within the range [-1, 1].
    fn safe_asin(self) -> Self;

    /// Calculates the arccosine of the number, ensuring that the result is within the range [-1, 1].
    fn safe_acos(self) -> Self;

    /// Returns the next representable floating-point number smaller than the given number.
    fn next_float_down(self) -> Self;

    /// Returns the next representable floating-point number larger than the given number.
    fn next_float_up(self) -> Self;

    /// Converts the angle from degrees to radians.
    fn deg_to_rad(self) -> Self;

    /// Converts the angle from radians to degrees.
    fn rad_to_deg(self) -> Self;

    /// Calculates the sine of x divided by x.
    fn sin_x_over_x(self) -> Self;

    /// Performs linear interpolation between two values.
    ///
    /// The `self` parameter specifies the interpolation factor, which should be in the range [0, 1].
    /// The `a` and `b` parameters are the values to interpolate between.
    ///
    /// # Examples
    ///
    /// ```
    /// use math::float::lerp;
    ///
    /// let result = lerp(0.5, 0.0, 1.0);
    /// assert_eq!(result, 0.5);
    /// ```
    fn lerp(self, a: Self, b: Self) -> Self;

    /// Smoothly blends between 0 and 1 for a value in the range \[a, b\].
    ///
    /// The `a` and `b` parameters specify the values to interpolate between.
    /// Any `x` value less than `a` will return 0, and any `x` value greater than `b` will return 1.
    /// In between, the result is interpolated using a smooth cubic polynomial interpolation.
    ///
    /// # Examples
    ///
    /// ```
    /// use math::float::smooth_step;
    ///
    /// assert_eq!(smooth_step(0.5, 0.0, 1.0), 0.5);
    /// assert_eq!(smooth_step(-2.0, 0.0, 1.0), 0.0);
    /// assert_eq!(smooth_step(2.0, 0.0, 1.0), 1.0);
    /// ```
    fn smooth_step(self, a: Self, b: Self) -> Self;

    /// The largest representable value less than 1.0.
    ///
    /// f32::from_bits in const context is not yet stable: <https://github.com/rust-lang/rust/issues/72447>
    /// However, transmute is stable in const context, so we use that instead.
    const ONE_MINUS_EPSILON: Self;

    /// The mathematical constant π (pi).
    const PI: Self;

    /// The reciprocal of π (1/π).
    const INV_PI: Self;

    /// The reciprocal of 2π (1/(2π)).
    const INV_2PI: Self;

    /// The reciprocal of 4π (1/(4π)).
    const INV_4PI: Self;

    /// Half of π (π/2).
    const PI_OVER_2: Self;

    /// Quarter of π (π/4).
    const PI_OVER_4: Self;
}

impl FloatExt for Float {
    #[inline]
    fn safe_sqrt(self) -> Self {
        self.max(0.0).sqrt()
    }

    #[inline]
    fn safe_asin(self) -> Self {
        self.clamp(-1.0, 1.0).asin()
    }

    #[inline]
    fn safe_acos(self) -> Self {
        self.clamp(-1.0, 1.0).acos()
    }

    fn next_float_down(self) -> Self {
        if self.is_infinite() && self < 0.0 {
            return self;
        }

        if self == 0.0 {
            return -0.0;
        }

        let mut bits = self.to_bits();
        if self > 0.0 {
            bits -= 1;
        } else {
            bits += 1;
        }

        Self::from_bits(bits)
    }

    fn next_float_up(self) -> Self {
        if self.is_infinite() && self > 0.0 {
            return self;
        }

        if self == -0.0 {
            return 0.0;
        }

        let mut bits = self.to_bits();
        if self >= 0.0 {
            bits += 1;
        } else {
            bits -= 1;
        }

        Self::from_bits(bits)
    }

    #[inline]
    fn deg_to_rad(self) -> Self {
        self * (Self::PI / 180.0)
    }

    #[inline]
    fn rad_to_deg(self) -> Self {
        self * (180.0 / Self::PI)
    }

    #[inline]
    fn sin_x_over_x(self) -> Self {
        if 1.0 - self * self == 1.0 {
            return 1.0;
        }

        self.sin() / self
    }

    #[inline]
    fn lerp(self, a: Self, b: Self) -> Self {
        a * (1.0 - self) + b * self
    }

    fn smooth_step(self, a: Self, b: Self) -> Self {
        if a == b {
            if self < a {
                return 0.0;
            }
            return 1.0;
        }

        let t = ((self - a) / (b - a)).clamp(0.0, 1.0);

        t * t * (3.0 - 2.0 * t)
    }

    #[cfg(not(f64))]
    const ONE_MINUS_EPSILON: Float = unsafe { std::mem::transmute(0x3f7fffffu32) };

    #[cfg(f64)]
    const ONE_MINUS_EPSILON: Float = unsafe { std::mem::transmute(0x3fefffffffffffffu64) };

    #[allow(clippy::excessive_precision)]
    const PI: Float = 3.14159265358979323846;
    #[allow(clippy::excessive_precision)]
    const INV_PI: Float = 0.31830988618379067154;
    #[allow(clippy::excessive_precision)]
    const INV_2PI: Float = 0.15915494309189533577;
    #[allow(clippy::excessive_precision)]
    const INV_4PI: Float = 0.07957747154594766788;
    #[allow(clippy::excessive_precision)]
    const PI_OVER_2: Float = 1.57079632679489661923;
    #[allow(clippy::excessive_precision)]
    const PI_OVER_4: Float = 0.78539816339744830961;
}
