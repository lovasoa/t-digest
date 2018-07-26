# tdigest

This is an implementation of Ted Dunning's [t-digest](https://github.com/tdunning/t-digest/) in Rust.

The implementaion is based off [Derrick Burns' C++ implementation](https://github.com/derrickburns/tdigest).

## Example

```rust
extern crate tdigest;

fn main() {
    // Create the object that will hold the statistics.
    // Its size in memory depends on the compression factor.
    let compression = 1000.0;
    let mut t = tdigest::Tdigest::new(compression);
    
    // Add values. The size of t in memory
    // does not grow indefinitely as new values are added
    t.add(69.0);
    t.add(0.001);

    // Compute Quantiles
    println!("The median is approximately {}", t.quantile(0.50));
    println!("The 99th percentile is {}", t.quantile(0.99));

   // Compute cumulative distribution function
    println!("cdf(1) {}", t.cdf(1.0));
    println!("cdf(5) {}", t.cdf(5.0));
}
```

## TODO
- Implement MEAN, STDEV, COUNT, TOTAL
- Repeat test changes ... load digest by reading test files. 
- 
- Dump centroids
- Save centroids
- Reload centroids
- Read data file file
- Incorporate advice given by Ted Dunning himself for testing
  - Check out the tests for the Java version. 
  - Test very large (>100,000) data sets, uniform and very skewed distributions
  - Test very small (<100) data sets
  - Test sorted and reverse sorted (which can affect accuracy and size)
  - Test mixed continuous and discrete distributions
  - Test a set that a large number of samples at a single point and all other points uniform but on the same side of the mass point
  - Test discrete distributions
  - Look at quantiles versus computed values from the original data 
  - Look at number of centroids in the results
  - Look at error relative to minimum distance to 0 or 1
  - Verify that all centroids meet the size and delta k criteria
  - Verify that the sum of centroid weights equals the number of samples seen
  - Verify all the accuracies for direct insertion as well as for merging of multiple datasets
- Once the tests pass, create a pull request to link this project to Ted Dunning's t-digest for other Rustaceans to find
- Release the code as a cargo crate
