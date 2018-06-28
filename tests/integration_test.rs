extern crate tdigest;

use std::fs::File;
use std::io::BufReader;
use tdigest::Tdigest;

#[test]
fn integation_test_save_centroids() {
    let result: i32;
    let mut t = Tdigest::new(1000.0);
    t.add(469.20, 1.0);
    match t.save_centroids("tests/centroid_test.json".to_string()) {
        Ok(_) => {
            result = 0;
        }
        Err(_err) => {
            result = 1;
        }
    }
    assert_eq!(result, 0);
}

fn integration_test_digest(fspec: String) {
    let result: i32 = 0;
    let mut fileReader = BufReader::new(File::open(&fspec));
    let mut vector: Vec<isize> = vec![];

    for line in fileReader.Lines() {
        match line {
            Err(why) => {
                println!("{:?}", why);
                result = 1;
            }
            Ok(string) => match string.trim().parse::<isize>() {
                None => {
                    println!("Not a number!");
                    result = 1;
                }
                Some(number) => println!("{}", number),
            },
        }
    }
}

#[test]
fn integation_test_digests_with_datasets() {}

//    let mut fileReader = BufReader::new(File::open(&path));
//    for line in fileReader.lines() {
//       match line {
//         Err(_why)   => { result = 0; },
//         Ok(string) => match string.trim().parse::<f64>() {
//            None        => panic!("Not a number!"),
//           Some(number)=> { t.add(number, 1.0); count += 1; }
//      }
//     }
//    }
//}
