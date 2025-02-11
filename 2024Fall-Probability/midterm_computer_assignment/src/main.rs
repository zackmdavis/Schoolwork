use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Binomial;

use rand_distr::Uniform;

// homework #10

fn hw_integral_1a(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let integrand = |x: f64| -> f64 { (1. - x.powi(2)).powf(1.5) };
    (0..n).map(|_| integrand(rng.gen())).sum::<f64>() / n as f64
}

use rand_distr::Pareto;

fn hw_integral_1b(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let proposal_distribution = Pareto::new(1., 2.).expect("valid Pareto distribution");
    let importance_weight =
        |y: f64| -> f64 { 0.5 * y.powi(3) * (y - 1.) * (1. + (y - 1.).powi(2)).powi(-2) };
    (0..n)
        .map(|_| importance_weight(proposal_distribution.sample(&mut rng)))
        .sum::<f64>()
        / n as f64
}

use rand_distr::StandardNormal;

fn hw_integral_1c(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let importance_weight = |x: f64| -> f64 {
        (2. * std::f64::consts::PI).sqrt() * (-x.powi(4) + 0.5 * x.powi(2) + 2.).exp()
    };
    (0..n)
        .map(|_| importance_weight(rng.sample(StandardNormal)))
        .sum::<f64>()
        / n as f64
}

use rand_distr::Exp;
use statrs::function::gamma::gamma;

fn hw_integral_2(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let proposal_distribution = Exp::new(0.5).expect("valid exponential distribution");
    let importance_weight =
        |x: f64| -> f64 { (-12.5_f64).exp() * (x + 25.).sqrt() / (2_f64.sqrt() * gamma(1.5)) };
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

fn integral_1b(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let proposal_distribution = Pareto::new(1., 5.).expect("valid Pareto distribution");
    let importance_weight =
        |y: f64| -> f64 { 0.2 * y.powi(6) * (y - 1.) * (2. + (y - 1.).powi(5)).powi(-4) };
    (0..n)
        .map(|_| importance_weight(proposal_distribution.sample(&mut rng)))
        .sum::<f64>()
        / n as f64
}


fn integral_1c(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let proposal_distribution = Pareto::new(0.5, 5.).expect("valid Pareto distribution");
    let importance_weight = |x: f64| -> f64 {
        (128./5.) * x.powi(7) * (2. + 2. * x.powi(3)).powi(-4)
    };
    (0..n)
        .map(|_| importance_weight(proposal_distribution.sample(&mut rng)))
        .sum::<f64>()
        / n as f64
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

    println!("{}", integral_1c(100));
    println!("{}", integral_1c(1000));
    println!("{}", integral_1c(10000));

    // println!("{:?}", question_2b());
    // println!("{:?}", question_2c());
    // println!("{:?}", question_2d());
}
