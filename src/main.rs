use ::dft::dft;
use ::dft::fft;
use ::dft::ntt;

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7];

    println!(
        "{:?}",
        dft::convolution(&a, &b)
            .into_iter()
            .map(|v| v.round() as i32)
            .collect::<Vec<_>>()
    );

    println!(
        "{:?}",
        fft::convolution(&a, &b)
            .iter()
            .map(|v| v.round() as i32)
            .collect::<Vec<_>>()
    );

    println!("{:?}", ntt::convolution(&a, &b));
}
