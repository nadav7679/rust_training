use std::env;

fn main() {
    // Parsing input
    let mut args = env::args().skip(1);
    let k: usize = args.next().unwrap().parse().unwrap();
    let args: Vec<f32> = args.map(|x| x.parse().unwrap()).collect();

    // println!("k is {:#} and args is {:?}", k, args);

    let n: usize = args.len();
    let mut avg = vec![0 as f32; n - k + 1];

    avg[0] = args[..k].into_iter().sum();
    avg[0] /= k as f32;

    let mut prev = avg[0];
    for i in 1..=n - k {
        prev += (args[i + k - 1] - args[i - 1]) / (k as f32);
        avg[i] = prev;
    }
}
