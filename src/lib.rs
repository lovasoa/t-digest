extern crate quickersort;

struct Centroid {
    mean: f64,
    weight: f64,
    sort_key1: isize,
    sort_key2: isize,
}

impl Centroid {
    fn to_string(&self) -> String {
        format!(
            "{{\"mean\": \"{mean}\",\"weight\": \"{weight}\"}}",
            mean = self.mean,
            weight = self.weight
        )
    }

    fn add(&mut self, r: &Centroid) -> String {
        if r.weight < 0.0 {
            return "centroid weight cannot be less than zero".to_string();
        }
        if self.weight != 0.0 {
            self.weight += r.weight;
            self.mean += r.weight * (r.mean - self.mean) / self.weight;
        } else {
            self.weight = r.weight;
            self.mean = r.mean;
        }
        return "".to_string();
    }

    fn clone(&self) -> Centroid {
        Centroid {
            mean: self.mean,
            weight: self.weight,
            sort_key1: self.mean.floor() as isize,
            sort_key2: self.mean.fract() as isize,
        }
    }
}

impl CentroidList {
    pub fn len(&self) -> usize {
        self.cvect.len()
    }
}

struct CentroidList {
    cvect: Vec<Centroid>,
}

pub struct Tdigest {
    compression: f64,
    max_processed: usize,
    max_unprocessed: usize,
    processed: CentroidList,
    unprocessed: CentroidList,
    cumulative: Vec<f64>,
    processed_weight: f64,
    unprocessed_weight: f64,
    min: f64,
    max: f64,
    mean: f64,
    stdev: f64,
    count: usize,
    total: f64,
}

impl Tdigest {
    pub fn new(set_compression: f64) -> Tdigest {
        Tdigest {
            compression: set_compression,
            max_processed: processed_size(0, set_compression),
            max_unprocessed: unprocessed_size(0, set_compression),
            processed: CentroidList {
                cvect: Vec::<Centroid>::new(),
            },
            unprocessed: CentroidList {
                cvect: Vec::<Centroid>::new(),
            },
            cumulative: Vec::<f64>::new(),
            processed_weight: 0.0,
            unprocessed_weight: 0.0,
            min: ::std::f64::MAX,
            max: -::std::f64::MAX,
            mean: 0.0,
            stdev: 0.0,
            count: 0,
            total: 0.0,
        }
    }

    fn process(&mut self) {
        let mut sd_m: f64 = 0.0;
        let mut sd_s: f64 = 0.0;
        let mut sd_k: f64 = 1.0;
        self.count = 0;
        self.stdev = 0.0;
        if self.unprocessed.len() > 0 || self.processed.len() > self.max_processed {
            // Append all processed centroids to the unprocessed list and sort by mean
            self.unprocessed.cvect.append(&mut self.processed.cvect);
            ::quickersort::sort_by_key(&mut self.unprocessed.cvect, |a| (a.sort_key1, a.sort_key2));

            // Reset processed list with first unprocessed centroid
            self.processed.cvect.clear();
            self.processed.cvect.push(self.unprocessed.cvect[0].clone());

            self.processed_weight += self.unprocessed_weight;
            self.unprocessed_weight = 0.0;
            let mut so_far: f64 = self.unprocessed.cvect[0].weight;
            let mut limit: f64 = self.processed_weight * self.integrated_q(1.0);
            let mut projected: f64;
            let mut k1: f64;
            let mut idx: usize;
            let mut rec: i32 = 0;
            for centroid in self.unprocessed.cvect.iter() {
                // update the count
                self.count += centroid.weight.round() as usize;

                // update the Welford standard deviation totals
                for _ in 0..centroid.weight.round() as usize {
                    let sd_tmp: f64 = sd_m;
                    sd_m += (centroid.mean - sd_tmp) / sd_k;
                    sd_s += (centroid.mean - sd_tmp) * (centroid.mean - sd_m);
                    sd_k += 1.0;
                }

                // do the regular centroid processing
                if rec == 0 {
                    // skip the first unprocessed centroid, emulating range [1:]
                    rec += 1;
                    continue;
                }
                projected = so_far + centroid.weight;
                if projected <= limit {
                    so_far = projected;
                    idx = self.processed.len() - 1;
                    self.processed.cvect[idx].add(centroid);
                } else {
                    k1 = self.integrated_location(so_far / self.processed_weight);
                    limit = self.processed_weight * self.integrated_q(k1 + 1.0);
                    so_far += centroid.weight;

                    self.processed.cvect.push(Centroid {
                        mean: centroid.mean,
                        weight: centroid.weight,
                        sort_key1: centroid.sort_key1,
                        sort_key2: centroid.sort_key2,
                    });
                }
            }

            self.min = self.min.min(self.processed.cvect[0].mean);
            self.max = self.max
                .max(self.processed.cvect[self.processed.len() - 1].mean);
            self.update_cumulative();
            self.unprocessed.cvect.clear();
        }

        // Finalise the standard deviation
        self.stdev = (sd_s / (sd_k - 2.0)).sqrt();
    }

    fn add_centroid(&mut self, c: Centroid) {
        self.unprocessed.cvect.push(Centroid {
            mean: c.mean,
            weight: c.weight,
            sort_key1: c.sort_key1,
            sort_key2: c.sort_key2,
        });
        self.unprocessed_weight += c.weight;

        if self.processed.len() > self.max_processed
            || self.unprocessed.len() > self.max_unprocessed
            {
                self.process();
            }
    }

    /// Take a new value with a weight.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = tdigest::Tdigest::new(1000.0);
    ///
    /// t.add_weighted(5.0, 10.0);
    ///
    /// for i in 0..10 {t.add(5.0)}; // equivalent
    /// ```
    pub fn add_weighted(&mut self, x: f64, w: f64) {
        if !x.is_nan() {
            self.add_centroid(Centroid {
                mean: x,
                weight: w,
                sort_key1: x.floor() as isize,
                sort_key2: x.fract() as isize,
            });
        }
    }

    /// Take a new value into account in the statistics
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = tdigest::Tdigest::new(1000.0);
    /// t.add(5.0);
    /// ```
    pub fn add(&mut self, x: f64) {
        self.add_weighted(x, 1.0);
    }

    fn update_cumulative(&mut self) {
        self.cumulative = Vec::<f64>::new();
        let mut prev: f64 = 0.0;
        let mut cur: f64;
        for centroid in self.processed.cvect.iter() {
            cur = centroid.weight;
            self.cumulative.push(prev + cur / 2.0);
            prev += cur;
        }
        self.cumulative.push(prev);
    }

    fn integrated_q(&self, k: f64) -> f64 {
        ((k.min(self.compression) * ::std::f64::consts::PI / self.compression
            - ::std::f64::consts::PI / 2.0)
            .sin() + 1.0) / 2.0
    }

    fn integrated_location(&self, q: f64) -> f64 {
        self.compression * ((2.0 * q - 1.0).asin() + ::std::f64::consts::PI / 2.0)
            / ::std::f64::consts::PI
    }

    pub fn cdf(&mut self, x: f64) -> f64 {
        let width: f64;
        self.process();
        match self.processed.cvect.len() {
            0 => return 0.0,
            1 => {
                width = self.max - self.min;
                if x <= self.min {
                    return 0.0;
                }
                if x >= self.max {
                    return 1.0;
                }
                if (x - self.min) <= width {
                    // min and max are too close together to do any viable interpolation
                    return 0.5;
                }
                return (x - self.min) / width;
            }
            _ => (),
        }

        if x <= self.min {
            return 0.0;
        }
        if x >= self.max {
            return 1.0;
        }
        let m0: f64 = self.processed.cvect[0].mean;
        // Left Tail
        if x <= m0 {
            if m0 - self.min > 0.0 {
                return (x - self.min) / (m0 - self.min) * self.processed.cvect[0].weight
                    / self.processed_weight / 2.0;
            }
            return 0.0;
        }
        // Right Tail
        let mn: f64 = self.processed.cvect[self.processed.len() - 1].mean;
        if x >= mn {
            if self.max - mn > 0.0 {
                return 1.0 - (self.max - x) / (self.max - mn)
                    * self.processed.cvect[self.processed.len() - 1].weight
                    / self.processed_weight / 2.0;
            }
            return 1.0;
        }

        // ?? Search the processed vector for the first Centroid with (mean > x)
        let mut idx: i32 = -1;
        for centroid in self.processed.cvect.iter() {
            idx += 1;
            if centroid.mean > x {
                break;
            }
        }

        let upper: usize = idx as usize;
        let z1: f64 = x - self.processed.cvect[upper - 1].mean;
        let z2: f64 = self.processed.cvect[upper].mean - x;
        return weighted_average(self.cumulative[upper - 1], z2, self.cumulative[upper], z1)
            / self.processed_weight;
    }

    //   fn add_centroid_list(&mut self, clist: CentroidList) {
    //       let mut available: usize = self.max_unprocessed - self.unprocessed.len();
    //       for centroid in clist.cvect.iter() {
    //           if available > 0 {
    //               self.add_centroid(Centroid {
    //                   mean: centroid.mean,
    //                   weight: centroid.weight,
    //                   sort_key1: centroid.sort_key1,
    //                   sort_key2: centroid.sort_key2,
    //              });
    //          }
    //          available -= 1;
    //      }
    //  }

    pub fn quantile(&mut self, q: f64) -> f64 {
        self.process();
        if q < 0.0 || q > 1.0 || self.processed.len() == 0 {
            return ::std::f64::NAN;
        }
        if self.processed.len() == 1 {
            return self.processed.cvect[0].mean;
        }

        let index: f64 = q * self.processed_weight;
        if index < self.processed.cvect[0].weight / 2.0 {
            return self.min
                + 2.0 * index / self.processed.cvect[0].weight
                * (self.processed.cvect[0].mean - self.min);
        }

        let mut lower: usize = self.cumulative.len() - 1;
        for idx in 0..self.cumulative.len() - 1 {
            if self.cumulative[idx] >= index {
                lower = idx as usize;
                break;
            }
        }

        if lower + 1 != self.cumulative.len() {
            let z1: f64 = index - self.cumulative[lower - 1];
            let z2: f64 = self.cumulative[lower] - index;
            return weighted_average(
                self.processed.cvect[lower - 1].mean,
                z2,
                self.processed.cvect[lower].mean,
                z1,
            );
        }

        let z1: f64 = index - self.processed_weight - self.processed.cvect[lower - 1].weight / 2.0;
        let z2: f64 = (self.processed.cvect[lower - 1].weight / 2.0) - z1;
        return weighted_average(
            self.processed.cvect[self.processed.len() - 1].mean,
            z1,
            self.max,
            z2,
        );
    }

    pub fn list_centroids(&mut self) -> String {
        self.process();
        let mut result = "[".to_string();
        let mut rec: i32 = 0;
        for centroid in self.processed.cvect.iter() {
            if rec == 0 {
                rec = 1;
                result = result + &centroid.to_string();
            } else {
                result = result + &",".to_string() + &centroid.to_string();
            }
        }
        result = result + &"]".to_string();
        return result;
    }

    pub fn save_centroids(&mut self, fspec: String) -> std::io::Result<()> {
        ::std::fs::write(fspec, self.list_centroids())
    }

    pub fn count(&self) -> usize {
        return self.count;
    }

    pub fn mean(&self) -> f64 {
        return self.mean;
    }

    pub fn stdev(&self) -> f64 {
        return self.stdev;
    }

    pub fn total(&self) -> f64 {
        return self.total;
    }
}

fn weighted_average(x1: f64, w1: f64, x2: f64, w2: f64) -> f64 {
    if x1 <= x2 {
        weighted_average_sorted(x1, w1, x2, w2);
    }
    weighted_average_sorted(x2, w2, x1, w1)
}

fn weighted_average_sorted(x1: f64, w1: f64, x2: f64, w2: f64) -> f64 {
    x1.max(x2.min((x1 * w1 + x2 * w2) / (w1 + w2)))
}

fn processed_size(size: usize, compression: f64) -> usize {
    if size == 0 {
        return (2.0 * compression.ceil()) as usize;
    }
    size
}

fn unprocessed_size(size: usize, compression: f64) -> usize {
    if size == 0 {
        return (8.0 * compression.ceil()).floor() as usize;
    }
    size
}

//#[cfg(test)]
//#[test]
