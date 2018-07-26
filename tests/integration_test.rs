extern crate tdigest;

use std::fs::File;
use std::io::BufReader;
use tdigest::Tdigest;

#[test]
fn integration_test_save_centroids() {
    let mut t = Tdigest::new(1000.0);
    t.add(469.20, 1.0);
    match t.save_centroids("tests/centroid_test.json".to_string()) {
        Ok(_) => (),
        Err(_err) => {
            panic!("Failed to save centroid");
        }
    }
}

fn integration_test_digest(fname: String) {
    // Load the statistics file ... the test 'answers', if you will, generated with R
    let fspec = format!("{}.sta", fname);
    let f = File::open(&fspec);
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("Failed to open dataset statistics check file {}: {}", fspec, e)
    };
}

#[test]
fn integation_test_digests_with_datasets() {
    integration_test_digest("data/large-normal".to_string());
}
