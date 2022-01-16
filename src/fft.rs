use crate::complex::*;
use crate::util::split_at_mut;

fn fft_inner(f: &mut [Complex], n: i64, exp: i64, ws: &mut [Complex]) {
    if n == 1 {
        return;
    }

    let (f0, f1) = split_at_mut(ws, (n >> 1) as usize);
    for i in 0..n as usize {
        if i % 2 == 0 {
            f0[i >> 1] = f[i];
        } else {
            f1[i >> 1] = f[i];
        }
    }

    let (ws0, ws1) = split_at_mut(f, (n >> 1) as usize);

    fft_inner(f0, n >> 1, exp, ws0);
    fft_inner(f1, n >> 1, exp, ws1);

    let z = zeta(exp, n);
    let mut x = Complex::new(1.0, 0.0);
    for i in 0..n {
        f[i as usize] = f0[(i % (n >> 1)) as usize] + x * f1[(i % (n >> 1)) as usize];
        x = x * z;
    }
}

pub fn fft<T>(f: &[T], n: i64) -> Vec<Complex>
where
    T: Into<Complex> + Copy,
{
    let mut f = f.iter().map(|v| (*v).into()).collect::<Vec<_>>();
    f.resize(n as usize, Complex::zero());
    let mut ws = vec![Complex::zero(); n as usize];
    fft_inner(&mut f, n, 1, &mut ws);
    f
}

pub fn inverse_fft<T>(f: &[T], n: i64) -> Vec<Complex>
where
    T: Into<Complex> + Copy,
{
    let mut f = f.iter().map(|v| (*v).into()).collect::<Vec<_>>();
    f.resize(n as usize, Complex::zero());
    let mut ws = vec![Complex::zero(); n as usize];
    fft_inner(&mut f, n, -1, &mut ws);
    for v in f.iter_mut() {
        *v = *v / n as f64;
    }
    f
}

pub fn convolution<T>(a: &[T], b: &[T]) -> Vec<f64>
where
    T: Into<Complex> + Copy,
{
    let m = (a.len() + b.len() - 1) as i64;
    let mut n = 1;
    while n < m {
        n <<= 1;
    }

    let g = fft(a, n);
    let h = fft(b, n);
    let f = (0..n)
        .map(|i| g[i as usize] * h[i as usize])
        .collect::<Vec<_>>();

    inverse_fft(&f, n)
        .iter()
        .take(m as usize)
        .map(|c| c.re)
        .collect()
}
