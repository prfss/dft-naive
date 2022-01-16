use crate::util::split_at_mut;
use num::{FromPrimitive, Integer, ToPrimitive};

const MOD: i64 = 998_244_353;
const PRIM_ROOT: i64 = 3;

pub trait NttElem: Integer + FromPrimitive + ToPrimitive {}

impl NttElem for i32 {}
impl NttElem for u32 {}
impl NttElem for i64 {}
impl NttElem for u64 {}

pub fn mod_pow(a: i64, b: i64, m: i64) -> i64 {
    if b == 0 {
        1
    } else {
        let ret = mod_pow(a, b / 2, m);
        let ret = ret * ret % m;
        if b % 2 == 1 {
            a * ret % m
        } else {
            ret
        }
    }
}

fn ntt_inner(f: &mut [i64], n: i64, n_root: i64, ws: &mut [i64]) {
    if n == 1 {
        return;
    }

    let (f0, f1) = split_at_mut(ws, (n >> 1) as usize);
    let mut b = true;
    for i in 0..n as usize {
        if b {
            f0[i >> 1] = f[i];
        } else {
            f1[i >> 1] = f[i];
        }

        b = !b;
    }

    let (ws0, ws1) = split_at_mut(f, (n >> 1) as usize);
    ntt_inner(f0, n >> 1, n_root * n_root % MOD, ws0);
    ntt_inner(f1, n >> 1, n_root * n_root % MOD, ws1);

    let mut g = 1;
    for i in 0..n {
        let j = (i & ((n >> 1) - 1)) as usize;
        f[i as usize] = f0[j] + g * f1[j] % MOD;
        if f[i as usize] >= MOD {
            f[i as usize] -= MOD;
        }
        g = g * n_root % MOD;
    }
}

fn modulo<T: NttElem>(f: &[T]) -> Vec<i64> {
    f.iter()
        .map(|v| v.mod_floor(&T::from_i64(MOD).unwrap()).to_i64().unwrap())
        .collect()
}

pub fn ntt<T: NttElem>(f: &[T], n: i64, n_root: i64) -> Vec<i64> {
    let mut f = modulo(f);
    f.resize(n as usize, 0);
    let mut ws = vec![0; n as usize];
    ntt_inner(&mut f, n, n_root, &mut ws);
    f
}

pub fn inverse_ntt<T: NttElem>(f: &[T], n: i64, n_root: i64) -> Vec<i64> {
    let mut f = modulo(f);
    f.resize(n as usize, 0);
    let d = mod_pow(n, MOD - 2, MOD);
    let n_root = mod_pow(n_root, MOD - 2, MOD);
    let mut ws = vec![0; n as usize];
    ntt_inner(&mut f, n, n_root, &mut ws);
    for v in f.iter_mut() {
        *v = *v * d % MOD;
    }
    f
}

pub fn convolution<T: NttElem>(a: &[T], b: &[T]) -> Vec<T> {
    let m = (a.len() + b.len() - 1) as i64;
    let mut n = 1;
    while n < m {
        n <<= 1;
    }

    assert_eq!((MOD - 1) % n, 0);

    let n_root = mod_pow(PRIM_ROOT, (MOD - 1) / n, MOD);

    let g = ntt(a, n, n_root);
    let h = ntt(b, n, n_root);

    let f = (0..n)
        .map(|i| g[i as usize] * h[i as usize] % MOD)
        .collect::<Vec<_>>();

    inverse_ntt(&f, n, n_root)
        .into_iter()
        .take(m as usize)
        .map(|v| T::from_i64(v).unwrap())
        .collect()
}
