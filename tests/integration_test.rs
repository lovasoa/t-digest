extern crate tdigest;

#[macro_use]
extern crate assert_approx_eq;

use tdigest::Tdigest;

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