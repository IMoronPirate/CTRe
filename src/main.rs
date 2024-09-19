use ctre::kde::{KDE, silverman_bandwith};
use rand::distributions::Distribution;
use rand::thread_rng;
use statrs::distribution::Normal;

fn main() {
    let data = vec![0.5, 0.6, 0.7, 0.8];
    let bandwith = silverman_bandwith(&data);
    let components = data
        .iter()
        .map(|xi| Normal::new(*xi, bandwith).unwrap())
        .collect::<Vec<_>>();
    let mykde = KDE::new(
        &vec![1.0 / data.len() as f64; data.len()], 
        &components
    );

    println!(
        "x = {:?};",
        (0..100).map(|i| i as f64 / 100.0).collect::<Vec<_>>()
    );
    println!(
        "y = {:?};",
        (0..100)
            .map(|i| mykde.pdf(i as f64 / 100.0))
            .collect::<Vec<_>>()
    );

    let mut rng = thread_rng();
}
