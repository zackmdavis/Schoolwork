use rand::distributions::{Distribution, Uniform};
use rand_distr::Gamma;

mod rare_events;

fn my_first_integrand(x: f64) -> f64 {
     x * (4. - x.powi(2)).powf(-0.5)
}

fn integrate(integrand: &dyn Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let uniform = Uniform::from(a..b);
    let mut rng = rand::thread_rng();
    let samples = (0..n).map(|_| integrand(uniform.sample(&mut rng)));
    let mean = samples.sum::<f64>() / n as f64;
    (b - a) * mean
}


fn exercise_1(trials: usize) -> f64 {
    let mut rng = rand::thread_rng();

    let arrival_time_distribution = Uniform::from((0.)..(60.));

    let mut rendezvous_count = 0;
    for _ in 0..trials {
        let mut arrivals = (0..2).map(|_| arrival_time_distribution.sample(&mut rng)).collect::<Vec<_>>();
        arrivals.sort_by(|a, b| a.partial_cmp(b).unwrap() );
        if arrivals[1] < arrivals[0] + 15. {
            rendezvous_count += 1;
        }
    }
    rendezvous_count as f64 / trials as f64
}

fn exercise_2_integrand_1(x: f64) -> f64 {
    x.powf(-0.5) * (4. - x.powi(5)).powf(-1./3.)
}

fn exercise_2_integral_1(n: usize) -> f64 {
    integrate(&exercise_2_integrand_1, 0., 3., n)
}

fn exercise_2_integrand_2(x: f64) -> f64 {
    x.sqrt() * (-x.powi(3) + 1.0).exp()
}

fn exercise_2_integral_2(n: usize) -> f64 {
    // 5 = ∞ for the purposes of an exponentially thin tail
    integrate(&exercise_2_integrand_2, 0., 5., n)
}

fn exercise_2_integrand_3(x: f64) -> f64 {
    (-x.powi(2) + 4.0).exp()
}

fn exercise_2_integral_3(n: usize) -> f64 {
    // 8 = ∞ as above
    integrate(&exercise_2_integrand_3, 3., 8., n)
}

fn exercise_2_integrand_4(x: f64) -> f64 {
    x.powi(2) * (-x.powi(6) - 1.).exp()
}

fn exercise_2_integral_4(n: usize) -> f64 {
    // 5 = ∞, −5 = −∞ as above
    integrate(&exercise_2_integrand_4, -5., 5., n)
}

fn exercise_3_inverse_cmf(p: f64) -> u32 {
    if p < 0.05 {
        0
    } else if p < 0.35 {
        2
    } else if p < 0.6 {
        4
    } else {
        5
    }
}

fn exercise_3_data() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let unit_uniform = Uniform::new(0., 1.);
    (0..100).map(|_| exercise_3_inverse_cmf(unit_uniform.sample(&mut rng))).collect::<Vec<_>>()
}

use plotters::prelude::*;

fn exercise_3_plot_histogram() -> Result<(), Box<dyn std::error::Error>> {
    let data = exercise_3_data();
    // following https://github.com/plotters-rs/plotters/blob/1f174c8b8e4b9/plotters/examples/histogram.rs
    let root = BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root).build_cartesian_2d((0u32..6u32).into_segmented(), 0u32..50u32)?;
    chart
        .configure_mesh()
        .draw()?;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;
    root.present().expect("file should write");
    Ok(())
}

// "Lt X ∼ Gamma(r = 3, λ = 2) . Use Monte Carlo simulations to estimate
// probability that P (X > 50), which is very small."
//
// I initially wrote code to do this the naïve way.
fn exercise_4_naïve() {
    let mut rng = rand::thread_rng();
    let distribution = Gamma::new(3., 2.).expect("gamma distribution");
    let mut hit_count = 0;
    let mut sample_count = 0;
    let mut max_sample = 0.;
    loop {
        let sample = distribution.sample(&mut rng);
        sample_count += 1;
        if sample > 50. {
            hit_count += 1;
        }
        if sample > max_sample {
            max_sample = sample;
        }
        if sample_count % 1000 == 0 {
            println!("after {} samples, {} hits", sample_count, hit_count);
            println!("largest sample was {}", max_sample);
        }
        if hit_count >= 2 {
            break;
        }
    }
    println!("finished with {} samples; implied probability {}", sample_count, 2./(sample_count as f64));
}

// "Exercise 5. Try to program baby example from the lecture."
//
// Uh, what baby example?! I watched the importance sampling lecture, and I did
// not see anything about babies.

fn worksheet_attempt() {
    // WolframAlpha says int_0^2 x * (4 - x**2)**(-1/2) dx = 2.
    // My code gives answers like 2.0004, 1.99172, 2.000, 2.0604, 1.9506
    // Checks out! ✓
    println!("{}", integrate(&my_first_integrand, 0., 2., 10000));

    println!("{}", exercise_1(10000)); // ≈ 0.43

    // WolframAlpha says int_0^3 sqrt(x) * (4 - x**5)**(-1/3) dx is a complex mumber?!
    // And my code is corrspondingly returning NaN.
    println!("{}", exercise_2_integral_1(10000));

    // At first I thought we were supposed to find some way to transform part
    // of the integrand into an exponential distribution, but I couldn't see
    // how to do it. (The obvious substitution u:= x^3+1 doesn't seem to help.)
    // The Claude LLM pointed out that the tail disperses so fast that we can
    // just use a finite cutoff ... but really, I should have watched the video
    // lecture on importance sampling first, because this sloppy approach is
    // not what was intended!

    // WolframAlpha says \int_0^\infty sqrt(x)exp(-x^3+1) ≈ 1.606.
    // My code gives answers like 1.612, 1.587, 1.648
    // ✓
    println!("{}", exercise_2_integral_2(10000));

    // WolframAlpha says \int_3^\infty exp(-x^2 + 4) dx ≈ 0.0010689.
    // My code says 0.00107, 0.00106, 0.0010338
    // ✓
    println!("{}", exercise_2_integral_3(10000));

    // WolframAlpha says "integrate x^2 * exp(-x^6 - 1) dx from -∞ to ∞" ≈ 0.21735.
    // My code says 0.223865, 0.207759, 0.2170699
    // ✓
    println!("{}", exercise_2_integral_4(10000));

    // (this successfully writes the histogram)
    exercise_3_plot_histogram().expect("it should work");

    // exercise_4_naïve();
    //
    // Example runs:
    // `finished with 38021344 samples; implied probability 0.00000005260203321586949`
    // `finished with 265294612 samples; implied probability 0.000000007538788612864855`
}


fn main() {
    rare_events::naïve_sample();
    println!("{}", rare_events::savvy_sample(10000));
}




0.000000006006 in 24 seconds, 0.000000004019 in 7 seconds, 0.000000004701 in 19 seconds
