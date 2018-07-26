extern crate tdigest;

#[macro_use]
extern crate assert_approx_eq;

#[macro_use]
extern crate quickcheck;

use tdigest::Tdigest;
use std::cmp::{max, min};

#[test]
fn linear() {
    let mut t = Tdigest::new(1000.0);
    let max = 100000;
    for i in 0..=max {
        t.add(i as f64);
    };
    assert_approx_eq!(max as f64/2.0, t.quantile(0.5), 100.);
    assert_approx_eq!(max as f64/10.0, t.quantile(0.1), 100.);
}

#[test]
fn empty() {
    assert!(Tdigest::new(1000.0).quantile(0.1).is_nan());
}

fn median_tdigest(xs: &Vec<f64>) -> f64 {
    let mut t = Tdigest::new(1000.0);
    for &x in xs {
        t.add(x);
    };
    t.quantile(0.5)
}

fn is_valid_median(xs: &Vec<f64>, median: f64) -> bool {
    let less = xs.iter().filter(|&&x| x <= median).count();
    let more = xs.iter().filter(|&&x| x >= median).count();

    min(less, more) <= 2 * max(less, more)
}

#[cfg(test)]
quickcheck! {
  fn test_valid_median(xs: Vec<f64>) -> bool {
      is_valid_median(&xs, median_tdigest(&xs))
  }
}
