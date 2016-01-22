// lib.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Jim Davies, Brian Gough
// Copyright (C) 2016 G.vd.Schoot
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
//

pub mod types;
use types::F64;

// mean

// mean calculates the arithmetic mean with the recurrence relation
pub fn mean<T: F64>(data: &[T]) -> f64 {
    let mut mean = 0.0;
    for (i, val) in data.iter().enumerate() {
        mean += (val.f64() - mean) / (i + 1) as f64;
    }
    mean
}

// absdev

pub fn absdev<T: F64>(data: &Vec<T>) -> f64 {
    absdev_mean(data, mean(data))
}

// absdev_mean finds the absolute deviation of the data interface
pub fn absdev_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let mut sum = 0.0;
    // the sum of the absolute deviations
    for val in data {
        sum += (val.f64() - mean).abs();
    }
    sum / data.len() as f64
}

// covariance

// takes a dataset and calculates the covariance
fn covariance_nonpub<T: F64>(data1: &Vec<T>, data2: &Vec<T>, mean1: f64, mean2: f64) -> f64 {
    let mut res = 0.0;
    // calculate the sum of the squares
    for i in 0..data1.len() {
        let delta1 = data1[i].f64() - mean1;
        let delta2 = data2[i].f64() - mean2;
        res += (delta1 * delta2 - res) / (i + 1) as f64;
    }
    res
}

pub fn covariance_mean<T: F64>(data1: &Vec<T>, data2: &Vec<T>, mean1: f64, mean2: f64) -> f64 {
    let n = data1.len();
    let covariance = covariance_nonpub(data1, data2, mean1, mean2);
    covariance * (n as f64) / (n - 1) as f64
}

pub fn covariance<T: F64>(data1: &Vec<T>, data2: &Vec<T>) -> f64 {
    let mean1 = mean(data1);
    let mean2 = mean(data2);
    covariance_mean(data1, data2, mean1, mean2)
}

// correlation()
// Calculate Pearson correlation = cov(X, Y) / (sigma_X * sigma_Y)
// This routine efficiently computes the correlation in one pass of the
// data and makes use of the algorithm described in:
//
// B. P. Welford, "Note on a Method for Calculating Corrected Sums of
// Squares and Products", Technometrics, Vol 4, No 3, 1962.
//
// This paper derives a numerically stable recurrence to compute a sum
// of products
//
// S = sum_{i=1..N} [ (x_i - mu_x) * (y_i - mu_y) ]
//
// with the relation
//
// S_n = S_{n-1} + ((n-1)/n) * (x_n - mu_x_{n-1}) * (y_n - mu_y_{n-1})
//
pub fn correlation<T: F64>(data1: &Vec<T>, data2: &Vec<T>) -> f64 {
    let mut sum_xsq = 0.0;
    let mut sum_ysq = 0.0;
    let mut sum_cross = 0.0;

    // Compute:
    // sum_xsq = Sum [ (x_i - mu_x)^2 ],
    // sum_ysq = Sum [ (y_i - mu_y)^2 ] and
    // sum_cross = Sum [ (x_i - mu_x) * (y_i - mu_y) ]
    // using the above relation from Welford's paper

    let mut mean_x = data1[0].f64();
    let mut mean_y = data2[0].f64();

    for i in 1..data1.len() {
        let ratio = i as f64 / (i + 1) as f64;
        let delta_x = data1[i].f64() - mean_x;
        let delta_y = data2[i].f64() - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / (i + 1) as f64;
        mean_y += delta_y / (i + 1) as f64;
    }

    sum_cross / (sum_xsq.sqrt() * sum_ysq.sqrt())
}

// kurtosis

pub fn kurtosis<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    let est_sd = sd_mean(data, mean);
    kurtosis_main_sd(data, mean, est_sd)
}

pub fn kurtosis_main_sd<T: F64>(data: &Vec<T>, mean: f64, sd: f64) -> f64 {
    let mut avg = 0.0;

    // calculate the fourth moment the deviations, normalized by the sd
    //
    // we use a recurrence relation to stable update a running value so
    // there aren't any large sums that can overflow

    for i in 0..data.len() {
        let x = (data[i].f64() - mean) / sd;
        avg += (x * x * x * x - avg) / (i + 1) as f64;
    }
    avg - 3.0 // makes kurtosis zero for a Gaussian
}

// lag-1

pub fn lag1autocorrelation<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    return lag1autocorrelation_mean(data, mean);
}

pub fn lag1autocorrelation_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let mut q = 0.0;
    let mut v = (data[0].f64() - mean) * (data[0].f64() - mean);

    for i in 1..data.len() {
        let delta0 = data[i - 1].f64() - mean;
        let delta1 = data[i].f64() - mean;
        q += (delta0 * delta1 - q) / (i + 1) as f64;
        v += (delta1 * delta1 - v) / (i + 1) as f64;
    }
    q / v // r1
}

// median

// MedianFromSortedData calculates the median of the sorted data.
// Note that the function doesn't check wheather the data is actually sorted.
pub fn median_from_sorted_data<T: F64>(sorted_data: &Vec<T>) -> f64 {
    let len = sorted_data.len();
    let lhs = (len - 1) / 2;
    let rhs = len / 2;

    if len == 0 {
        return 0.0;
    }

    if lhs == rhs {
        sorted_data[lhs].f64()
    } else {
        (sorted_data[lhs].f64() + sorted_data[rhs].f64()) / 2.0
    }
}

// minmax

// Max finds the first largest member and the members position within the data
pub fn max<T: F64>(data: &Vec<T>) -> (f64, usize) {
    let mut max = data[0].f64();
    let mut max_index = 0;

    for (i, val) in data.iter().enumerate() {
        let xi = val.f64();

        if xi > max {
            max = xi;
            max_index = i;
        }

        if xi.is_nan() {
            max = xi;
            max_index = i;
            return (max, max_index);
        }
    }
    (max, max_index)
}

// Min finds the first smallest member and the members position within the data
pub fn min<T: F64>(data: &Vec<T>) -> (f64, usize) {
    let mut min = data[0].f64();
    let mut min_index = 0;

    for (i, val) in data.iter().enumerate() {
        let xi = val.f64();

        if xi < min {
            min = xi;
            min_index = i;
        }

        if xi.is_nan() {
            min = xi;
            min_index = i;
            return (min, min_index);
        }
    }
    (min, min_index)
}

// Minmax finds the first smallest and largest members and
// the members positions within the data
pub fn minmax<T: F64>(data: &Vec<T>) -> (f64, u32, f64, u32) {
    let mut min_index: u32 = 0;
    let mut max_index: u32 = 0;
    let mut min = data[0].f64();
    let mut max = data[0].f64();

    for i in 0..data.len() {
        let xi = data[i].f64();

        if xi < min {
            min = xi;
            min_index = i as u32;
        }

        if xi > max {
            max = xi;
            max_index = i as u32;
        }

        if xi.is_nan() {
            min = xi;
            max = xi;
            min_index = i as u32;
            max_index = i as u32;
            break;
        }
    }
    (min, min_index, max, max_index)
}

// pvariance

// p_variance finds the pooled variance of two datasets
pub fn p_variance<T: F64>(data1: &Vec<T>, data2: &Vec<T>) -> f64 {
    let n1 = data1.len();
    let n2 = data2.len();

    let var1 = variance(data1);
    let var2 = variance(data2);

    (((n1 - 1) as f64 * var1) + ((n2 - 1) as f64 * var2)) / (n1 + n2 - 2) as f64
}

// quantiles

// QuantileFromSortedData performs the quantile function, also called percent
// point function or inverse cumulative distribution function, on the sorted data.
// Note that the function doesn't check wheather the data is actually sorted.
pub fn quantile_from_sorted_data<T: F64>(sorted_data: &Vec<T>, f: f64) -> f64 {
    let n = sorted_data.len() as i32;
    let index = f * (n - 1) as f64;
    let lhs = index as i32;
    let delta = index - (lhs as f64);

    if n == 0 {
        return 0.0;
    }

    if lhs == n - 1 {
        return sorted_data[lhs as usize].f64();
    } else {
        return (1.0 - delta) * sorted_data[lhs as usize].f64() +
               delta * sorted_data[lhs as usize + 1].f64();
    }
}

// skew

pub fn skew<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    let sd = sd_mean(data, mean);
    skew_mean_sd(data, mean, sd)
}

// Skew_mean_sd calculates the skewness of a dataset
pub fn skew_mean_sd<T: F64>(data: &Vec<T>, mean: f64, sd: f64) -> f64 {
    let mut skew = 0.0;
    for (i, val) in data.iter().enumerate() {
        let x = (val.f64() - mean) / sd;
        skew += (x * x * x - skew) / (i + 1) as f64;
    }
    skew
}

// ttest

// runs a t-test between two datasets representing independent
// samples. Tests to see if the difference between means of the
// samples is different from zero.
pub fn t_test<T: F64>(data1: &Vec<T>, data2: &Vec<T>) -> f64 {
    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;

    let mean1 = mean(data1);
    let mean2 = mean(data2);

    let pv = p_variance(data1, data2);

    (mean1 - mean2) / (pv * ((1.0 / n1) + (1.0 / n2))).sqrt()
}

// variance

fn _variance<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let mut variance = 0.0;

    // calculate the sum of the squares
    for (i, val) in data.iter().enumerate() {
        let delta = val.f64() - mean;
        // TODO: long double for variance... How to implement in Rust?
        variance += ((delta * delta) - variance) / (i + 1) as f64;
    }
    variance
}

pub fn variance_with_fixed_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    _variance(data, mean)
}

pub fn sd_with_fixed_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    _variance(data, mean).sqrt()
}

pub fn variance_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let variance = _variance(data, mean);
    variance * (data.len() as f64) / (data.len() - 1) as f64
}

pub fn sd_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let variance = _variance(data, mean);
    (variance * (data.len() as f64) / (data.len() - 1) as f64).sqrt()
}

pub fn variance<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    variance_mean(data, mean)
}

pub fn sd<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    return sd_mean(data, mean);
}

// tss_mean takes a dataset and finds the sum of squares about the mean
pub fn tss_mean<T: F64>(data: &Vec<T>, mean: f64) -> f64 {
    let mut res = 0.0;

    // find the sum of the squares
    for val in data {
        let delta = val.f64() - mean;
        res += delta * delta;
    }
    res
}

pub fn tss<T: F64>(data: &Vec<T>) -> f64 {
    let mean = mean(data);
    return tss_mean(data, mean);
}

// wabsdev

pub fn w_absdev<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    w_absdev_mean(w, data, wmean)
}

// WAbsdev_mean calculates the weighted absolute deviation of a dataset
pub fn w_absdev_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let mut wabsdev = 0.0;
    let mut weight = 0.0;

    // calculate the sum of the absolute deviations
    for i in 0..data.len() {
        let wi = w[i].f64();

        if wi > 0.0 {
            let delta = (data[i].f64() - wmean).abs();
            weight += wi;
            wabsdev += (delta - wabsdev) * (wi / weight);
        }
    }
    wabsdev
}

// wkurtosis

pub fn w_kurtosis<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    let wsd = w_sd_mean(w, data, wmean);
    w_kurtosis_mean_sd(w, data, wmean, wsd)
}

// w_kurtosis_mean calculates the kurtosis of a dataset
pub fn w_kurtosis_mean_sd<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64, wsd: f64) -> f64 {
    let mut wavg = 0.0;
    let mut weight = 0.0;

    for (i, val) in data.iter().enumerate() {
        let wi = w[i].f64();

        if wi > 0.0 {
            let x = (val.f64() - wmean) / wsd;
            weight += wi;
            wavg += (x * x * x * x - wavg) * (wi / weight);
        }
    }
    wavg - 3.0 // makes kurtosis zero for a Gaussian
}

// wmean

// w_mean calculates the weighted arithmetic mean of a dataset
pub fn w_mean<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let mut wmean = 0.0;
    let mut weight = 0.0;

    for (i, val) in data.iter().enumerate() {
        let wi = w[i].f64();

        if wi > 0.0 {
            weight += wi;
            wmean += (val.f64() - wmean) * (wi / weight);
        }
    }
    wmean
}

// wskew

pub fn w_skew<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    let wsd = w_sd_mean(w, data, wmean);
    return w_skew_mean_sd(w, data, wmean, wsd);
}

// Compute the weighted skewness of a dataset
pub fn w_skew_mean_sd<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64, wsd: f64) -> f64 {
    let mut wskew = 0.0;
    let mut weight = 0.0;

    for (i, val) in data.iter().enumerate() {
        let wi = w[i].f64();

        if wi > 0.0 {
            let x = (val.f64() - wmean) / wsd;
            weight += wi;
            wskew += (x * x * x - wskew) * (wi / weight);
        }
    }
    wskew
}

// wvariance

fn wvariance<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let mut weight = 0.0;
    let mut wvariance = 0.0;

    // the sum of the squares
    for (i, val) in w.iter().enumerate() {
        let wi = val.f64();
        if wi > 0.0 {
            let delta = data[i].f64() - wmean;
            weight += wi;
            wvariance += (delta * delta - wvariance) * (wi / weight);
        }
    }
    wvariance
}

fn factor<T: F64>(w: &Vec<T>) -> f64 {
    let mut a = 0.0;
    let mut b = 0.0;

    // the sum of the squares
    for val in w {
        let wi = val.f64();
        if wi > 0.0 {
            a += wi;
            b += wi * wi;
        }
    }
    (a * a) / ((a * a) - b)
}

pub fn w_variance_with_fixed_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    wvariance(w, data, wmean)
}

pub fn wsd_with_fixed_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let wvariance = wvariance(w, data, wmean);
    wvariance.sqrt()
}

pub fn w_variance_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let variance = wvariance(w, data, wmean);
    let scale = factor(w);

    scale * variance
}

pub fn w_sd_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let variance = wvariance(w, data, wmean);
    let scale = factor(w);
    (scale * variance).sqrt()
}

pub fn w_sd<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    w_sd_mean(w, data, wmean)
}

pub fn w_variance<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    w_variance_mean(w, data, wmean)
}

// w_tss_mean takes a dataset and finds the weighted sum of squares about wmean
pub fn w_tss_mean<T: F64>(w: &Vec<T>, data: &Vec<T>, wmean: f64) -> f64 {
    let mut res = 0.0;

    // find the sum of the squares
    for (i, val) in data.iter().enumerate() {
        let wi = w[i].f64();
        if wi > 0.0 {
            let delta = val.f64() - wmean;
            res += wi * delta * delta;
        }
    }
    res
}

pub fn w_tss<T: F64>(w: &Vec<T>, data: &Vec<T>) -> f64 {
    let wmean = w_mean(w, data);
    return w_tss_mean(w, data, wmean);
}
