// f64_test.rs
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

mod gsl;

extern crate stat;
use stat::*;

use std::cmp::Ordering::Equal;
use std::f64;


#[test]
fn test_float64_slice() {
    // sample sets of doubles
    let slice_a = &[0.0421, 0.0941, 0.1064, 0.0242, 0.1331, 0.0773, 0.0243, 0.0815, 0.1186,
                    0.0356, 0.0728, 0.0999, 0.0614, 0.0479];

    let slice_b = &[0.1081, 0.0986, 0.1566, 0.1961, 0.1125, 0.1942, 0.1079, 0.1021, 0.1583,
                    0.1673, 0.1675, 0.1856, 0.1688, 0.1512];

    let slice_w = &[0.000, 0.000, 0.000, 3.000, 0.0000, 1.000, 1.000, 1.000, 0.000, 0.5000, 7.000,
                    5.000, 4.000, 0.123];

    let rel = 1.0e-10;

    {
        let mean = mean(slice_a);
        let expected = 0.0728;
        gsl::test_rel(mean, expected, rel, "mean()");
    }

    {
        let mean = mean(slice_a);
        let varc = variance_with_fixed_mean(slice_a, mean);
        let expected = 0.00113837428571429;
        gsl::test_rel(varc, expected, rel, "variance_with_fixed_mean");
    }

    {
        let mean = mean(slice_a);
        let varc = sd_with_fixed_mean(slice_a, mean);
        let expected = 0.0337398026922845;
        gsl::test_rel(varc, expected, rel, "sd_with_fixed_mean");
    }

    {
        let varc = variance(slice_b);
        let expected = 0.00124956615384615;
        gsl::test_rel(varc, expected, rel, "variance");
    }

    {
        let sd = sd(slice_a);
        let expected = 0.0350134479659107;
        gsl::test_rel(sd, expected, rel, "sd");
    }

    {
        let ss = tss(slice_b);
        let expected = 0.01624436;
        gsl::test_rel(ss, expected, rel, "tss");
    }

    {
        let mean = mean(slice_a);
        let ss = tss_mean(slice_a, mean);
        let expected = 1.59372400000000e-02;
        gsl::test_rel(ss, expected, rel, "tss_mean");
    }

    {
        let absdev = absdev(slice_a);
        let expected = 0.0287571428571429;
        gsl::test_rel(absdev, expected, rel, "absdev");
    }

    {
        let skew = skew(slice_a);
        let expected = 0.0954642051479004;
        gsl::test_rel(skew, expected, rel, "skew");
    }

    {
        let kurt = kurtosis(slice_a);
        let expected = -1.38583851548909;
        gsl::test_rel(kurt, expected, rel, "kurtosis");
    }

    {
        let wmean = w_mean(slice_w, slice_a);
        let expected = 0.0678111523670601;
        gsl::test_rel(wmean, expected, rel, "w_mean");
    }

    {
        let wmean = w_mean(slice_w, slice_a);
        let wvar = w_variance_with_fixed_mean(slice_w, slice_a, wmean);
        let expected = 0.000615793060878654;
        gsl::test_rel(wvar, expected, rel, "w_variance_with_fixed_mean");
    }

    {
        let est_wvar = w_variance(slice_w, slice_a);
        let expected = 0.000769562962860317;
        gsl::test_rel(est_wvar, expected, rel, "w_variance");
    }

    {
        let wsd = w_sd(slice_w, slice_a);
        let expected = 0.0277409978706664;
        gsl::test_rel(wsd, expected, rel, "w_sd");
    }

    {
        let wtss = w_tss(slice_w, slice_a);
        let expected = 1.39310864162578e-02;
        gsl::test_rel(wtss, expected, rel, "w_tss");
    }

    {
        let wmean = w_mean(slice_w, slice_a);
        let wtss = w_tss_mean(slice_w, slice_a, wmean);
        let expected = 1.39310864162578e-02;
        gsl::test_rel(wtss, expected, rel, "w_tss_mean");
    }

    {
        let wabsdev = w_absdev(slice_w, slice_a);
        let expected = 0.0193205027504008;
        gsl::test_rel(wabsdev, expected, rel, "w_absdev");
    }

    {
        let wskew = w_skew(slice_w, slice_a);
        let expected = -0.373631000307076;
        gsl::test_rel(wskew, expected, rel, "w_skew");
    }

    {
        let wkurt = w_kurtosis(slice_w, slice_a);
        let expected = -1.48114233353963;
        gsl::test_rel(wkurt, expected, rel, "w_kurtosis");
    }

    {
        let c = covariance(slice_a, slice_b);
        let expected = -0.000139021538461539;
        gsl::test_rel(c, expected, rel, "covariance");
    }

    {
        let r = correlation(slice_a, slice_b);
        let expected = -0.112322712666074171;
        gsl::test_rel(r, expected, rel, "correlation");
    }

    {
        let pv = p_variance(slice_a, slice_b);
        let expected = 0.00123775384615385;
        gsl::test_rel(pv, expected, rel, "p_variance");
    }

    {
        let t = t_test(slice_a, slice_b);
        let expected = -5.67026326985851;
        gsl::test_rel(t, expected, rel, "t_test");
    }

    {
        let (max, _) = max(slice_a);
        let expected = 0.1331;
        let str = format!("max ({} observed vs {} expected)", max, expected);
        gsl::test(max != expected, &str);
    }

    {
        let (min, _) = min(slice_a);
        let expected = 0.0242;
        let str = format!("min ({} observed vs {} expected)", min, expected);
        gsl::test(min != expected, &str);
    }

    {
        let (min, _, max, _) = minmax(slice_a);
        let expected_max = 0.1331;
        let expected_min = 0.0242;

        let str1 = format!("minmax max ({} observed vs {} expected)", max, expected_max);
        let str2 = format!("minmax min ({} observed vs {} expected)", min, expected_min);

        gsl::test(max != expected_max, &str1);
        gsl::test(min != expected_min, &str2);
    }

    {
        let (_, max_index) = max(slice_a);
        let expected = 4;
        let str = format!("max ({} observed vs {} expected)", max_index, expected);
        gsl::test(max_index != expected, &str);
    }

    {
        let (_, min_index) = min(slice_a);
        let expected = 3;
        let str = format!("min ({} observed vs {} expected)", min_index, expected);
        gsl::test(min_index != expected, &str);
    }

    {
        let (_, min_index, _, max_index) = minmax(slice_a);
        let expected_max_index = 4;
        let expected_min_index = 3;

        let str1 = format!("minmax max ({} observed vs {} expected)",
                           max_index,
                           expected_max_index);
        let str2 = format!("minmax min ({} observed vs {} expected)",
                           min_index,
                           expected_min_index);

        gsl::test(max_index != expected_max_index, &str1);
        gsl::test(min_index != expected_min_index, &str2);
    }

    let mut sorted = slice_a.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    {
        let median = median_from_sorted_data(&sorted);
        let expected = 0.07505;
        gsl::test_rel(median, expected, rel, "median_from_sorted_data (even)");
    }

    {
        let zeroth = quantile_from_sorted_data(&sorted, 0.0);
        let expected = 0.0242;
        gsl::test_rel(zeroth, expected, rel, "quantile_from_sorted_data (0)");
    }

    {
        let top = quantile_from_sorted_data(&sorted, 1.0);
        let expected = 0.1331;
        gsl::test_rel(top, expected, rel, "quantile_from_sorted_data (100)");
    }

    {
        let median = quantile_from_sorted_data(&sorted, 0.5);
        let expected = 0.07505;
        gsl::test_rel(median, expected, rel, "quantile_from_sorted_data (50even)");
    }

    // Test for IEEE handling - set third element to NaN

    let mut slice_ac = slice_a.clone();
    slice_ac[3] = f64::NAN;

    {
        let (max, max_index) = max(&slice_ac);
        let expected = f64::NAN;
        let expected_index = 3;

        let str1 = format!("max NaN ({} observed vs {} expected)", max, expected);
        gsl::test(!max.is_nan(), &str1);

        let str2 = format!("max NaN index ({} observed vs {} expected)",
                           max_index,
                           expected_index);
        gsl::test(max_index != expected_index, &str2);
    }

    {
        let (min, min_index) = min(&slice_ac);
        let expected = f64::NAN;
        let expected_index = 3;

        let str1 = format!("min NaN ({} observed vs {} expected)", min, expected);
        gsl::test(!min.is_nan(), &str1);

        let str2 = format!("min NaN index ({} observed vs {} expected)",
                           min_index,
                           expected_index);
        gsl::test(min_index != expected_index, &str2);
    }

    {
        let (min, min_index, max, max_index) = minmax(&slice_ac);
        let expected_max = f64::NAN;
        let expected_min = f64::NAN;
        let expected_max_index = 3;
        let expected_min_index = 3;

        let str1 = format!("minmax max NaN ({} observed vs {} expected)",
                           max,
                           expected_max);
        gsl::test(!max.is_nan(), &str1);
        let str2 = format!("minmax min NaN ({} observed vs {} expected)",
                           min,
                           expected_min);
        gsl::test(!min.is_nan(), &str2);

        let str3 = format!("minmax max index NaN ({} observed vs {} expected)",
                           max_index,
                           expected_max_index);
        gsl::test(max_index != expected_max_index, &str3);
        let str4 = format!("minmax min index NaN ({} observed vs {} expected)",
                           min_index,
                           expected_min_index);
        gsl::test(min_index != expected_min_index, &str4);
    }
}
