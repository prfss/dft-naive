use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl From<f64> for Complex {
    fn from(re: f64) -> Self {
        Complex::new(re, 0.0)
    }
}

impl From<i32> for Complex {
    fn from(re: i32) -> Self {
        Complex::new(re as f64, 0.0)
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl ops::Div<f64> for Complex {
    type Output = Self;
    fn div(self, d: f64) -> Self {
        Self {
            re: self.re / d,
            im: self.im / d,
        }
    }
}

pub fn zeta(i: i64, n: i64) -> Complex {
    let (im, re) = (i as f64 * 2.0 * std::f64::consts::PI / n as f64).sin_cos();
    Complex { re, im }
}
