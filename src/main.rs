extern crate tdigest;

use tdigest::Tdigest;

fn main() {
    let mut t = Tdigest::new(1000.0);

    t.add(469.20, 1.0);
    match t.save_centroids("/tmp/centroid.json".to_string()) {
        Ok(_) => (),
        Err(err) => eprintln!("That didn't work {:?}", err),
    }
    ::std::process::exit(0);

    // Compute Quantiles
    //    println!(" 0.5th percentile is {}", t.quantile(0.005));
    //    println!("   1st percentile is {}", t.quantile(0.01));
    //    println!("   5th percentile is {}", t.quantile(0.05));
    //    println!("  10th percentile is {}", t.quantile(0.10));
    //    println!("  25th percentile is {}", t.quantile(0.25));
    //    println!("  50th percentile is {}", t.quantile(0.50));
    //    println!("  75th percentile is {}", t.quantile(0.75));
    //    println!("  90th percentile is {}", t.quantile(0.90));
    //    println!("  95th percentile is {}", t.quantile(0.95));
    //    println!("  99th percentile is {}", t.quantile(0.99));
    //    println!("99.5th percentile is {}", t.quantile(0.995));

    // Compute CDFs
    //   println!("cdf(100) {}", t.cdf(100.0));
    //   println!("cdf(200) {}", t.cdf(200.0));
    //   println!("cdf(300) {}", t.cdf(300.0));
    //   println!("cdf(400) {}", t.cdf(400.0));
    //   println!("cdf(500) {}", t.cdf(500.0));

    // Print centroids
    //  println!("{}", t.list_centroids());
}
