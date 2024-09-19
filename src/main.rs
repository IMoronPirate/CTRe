use ctre::kde::KDE;
use statrs::distribution::Normal;

fn main() {
    let data = vec![0.5, 0.6, 0.7, 0.8];
    let mykde = KDE::new(&data, Normal::standard());

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
}
