use rand::distributions::Distribution;
use rand_distr::{Exp, Gamma};

pub fn naïve_sample() {
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
    println!(
        "finished with {} samples; implied probability {}",
        sample_count,
        2. / (sample_count as f64)
    );
}

pub fn savvy_sample(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let sampling_distribution = Exp::new(0.5).expect("exp(½)");
    let c = (-25.0f64).exp() / 8.;
    let h = |y: f64| (y + 50.).powi(2);
    let h_x_i = (0..n).map(|_| h(sampling_distribution.sample(&mut rng)));
    c * h_x_i.sum::<f64>() / n as f64
}

// Example naïve estimate: 0.00000001775706861623417
// Example savvy estimate: 0.000000004699980753128407
