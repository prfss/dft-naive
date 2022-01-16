use crate::complex::*;

pub fn dft<T>(f: &[T], n: i64) -> Vec<Complex>
where
    T: Into<Complex> + Copy,
{
    (0..n).map(|i| apply(f, n, i)).collect()
}

pub fn inverse_dft<T>(f: &[T], n: i64) -> Vec<Complex>
where
    T: Into<Complex> + Copy,
{
    (0..n).map(|i| apply(f, n, -i) / n as f64).collect()
}

pub fn apply<T>(f: &[T], n: i64, i: i64) -> Complex
where
    T: Into<Complex> + Copy,
{
    f.iter()
        .enumerate()
        .map(|(j, a)| zeta(i * j as i64, n) * (*a).into())
        .fold(Complex::zero(), |a, c| a + c)
}

pub fn convolution<T>(a: &[T], b: &[T]) -> Vec<f64>
where
    T: Into<Complex> + Copy,
{
    let n = (a.len() + b.len() - 1) as i64;
    let g = dft(a, n);
    let h = dft(b, n);
    let f = (0..n)
        .map(|i| g[i as usize] * h[i as usize])
        .collect::<Vec<_>>();

    inverse_dft(&f, n)
        .iter()
        .take(n as usize)
        .map(|c| c.re)
        .collect()
}
