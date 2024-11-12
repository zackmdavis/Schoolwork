use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Binomial;

use rand_distr::Uniform;

use rand_distr::Pareto;

// homework #10

fn hw_integral_1a(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let integrand = |x: f64| -> f64 { (1. - x.powi(2)).powf(1.5) };
    (0..n).map(|_| integrand(rng.gen())).sum::<f64>() / n as f64
}

fn hw_integral_1b(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let proposal_distribution = Pareto::new(1., 2.).expect("valid Pareto distribution");
    let importance_weight = |x: f64| -> f64 { 0.5 * x.powi(4) * (1. + x.powi(2)).powi(-2) };
    (0..n)
        .map(|_| importance_weight(proposal_distribution.sample(&mut rng)))
        .sum::<f64>()
        / n as f64
}

/// midterm assignment

fn integral_1a(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let integrand = |x: f64| -> f64 { (3. - x.powi(7)).powf(1.5) };
    (0..n).map(|_| integrand(rng.gen())).sum::<f64>() / n as f64
}

fn question_2b() -> (Vec<u64>, Vec<u64>) {
    let mut rng = rand::thread_rng();

    let x = Binomial::new(100, 0.03).expect("valid binomial distribution");
    let y = Binomial::new(100, 0.05).expect("valid binomial distribution");

    let xs = (0..1000).map(|_| x.sample(&mut rng)).collect::<Vec<_>>();
    let ys = (0..1000).map(|_| y.sample(&mut rng)).collect::<Vec<_>>();

    (xs, ys)
}

fn question_2c() -> (f64, f64) {
    let mut rng = rand::thread_rng();

    let x = Binomial::new(100, 0.03).expect("valid binomial distribution");
    let y = Binomial::new(100, 0.05).expect("valid binomial distribution");

    let xs_less_than_10 = (0..1000)
        .map(|_| x.sample(&mut rng))
        .filter(|n| *n < 10)
        .count() as f64;
    let ys_less_than_10 = (0..1000)
        .map(|_| y.sample(&mut rng))
        .filter(|n| *n < 10)
        .count() as f64;

    (xs_less_than_10 / 1000., ys_less_than_10 / 1000.)
}

fn question_2d() -> f64 {
    let mut rng = rand::thread_rng();

    let x = Binomial::new(100, 0.03).expect("valid binomial distribution");
    let y = Binomial::new(100, 0.05).expect("valid binomial distribution");

    let a_more_defective = (0..1000)
        .map(|_| x.sample(&mut rng) > y.sample(&mut rng))
        .filter(|p| *p)
        .count() as f64;

    a_more_defective / 1000.
}

fn main() {
    println!("Hello, world!");

    println!("{}", hw_integral_1b(100));
    println!("{}", hw_integral_1b(1000));
    println!("{}", hw_integral_1b(10000));

    // println!("{:?}", question_2b());
    // println!("{:?}", question_2c());
    // println!("{:?}", question_2d());
}
