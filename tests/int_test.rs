// int_test.rs
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

#[test]
fn test_i32_slice() {
    // sample sets of integers

    let slice_1 = &[1, 2, 3, 4, 5, 6];

    let slice_a = &[17, 18, 16, 18, 12, 20, 18, 20, 20, 22, 20, 10, 8, 12, 16, 16, 18, 20, 18, 21];

    let slice_b = &[19, 20, 22, 24, 10, 25, 20, 22, 21, 23, 20, 10, 12, 14, 12, 20, 22, 24, 23, 17];

    let rel = 1.0e-10;

    {
        let mean = mean(slice_a);
        let expected = 17.0;
        gsl::test_rel(mean, expected, rel, "mean(integer)");
    }

    {
        let mean = mean(slice_1);
        let expected = 3.5;
        gsl::test_rel(mean, expected, rel, "mean(fractional)");
    }

    {
        let mean = mean(slice_a);
        let variance = variance_with_fixed_mean(slice_a, mean);
        let expected = 13.7;
        gsl::test_rel(variance, expected, rel, "variance_with_fixed_mean");
    }

    {
        let mean = mean(slice_a);
        let sd = sd_with_fixed_mean(slice_a, mean);
        let expected = 3.70135110466435;
        gsl::test_rel(sd, expected, rel, "sd_with_fixed_mean");
    }

    {
        let variance = variance(slice_a);
        let expected = 14.4210526315789;
        gsl::test_rel(variance, expected, rel, "variance");
    }

    {
        let sd_est = sd(slice_a);
        let expected = 3.79750610685209;
        gsl::test_rel(sd_est, expected, rel, "sd");
    }

    {
        let absdev = absdev(slice_a);
        let expected = 2.9;
        gsl::test_rel(absdev, expected, rel, "absdev");
    }

    {
        let skew = skew(slice_a);
        let expected = -0.909355923168064;
        gsl::test_rel(skew, expected, rel, "skew");
    }

    {
        let kurt = kurtosis(slice_a);
        let expected = -0.233692524908094;
        gsl::test_rel(kurt, expected, rel, "kurtosis");
    }

    {
        let c = covariance(slice_a, slice_b);
        let expected = 14.5263157894737;
        gsl::test_rel(c, expected, rel, "covariance");
    }

    {
        let r = correlation(slice_a, slice_b);
        let expected = 0.793090350710101;
        gsl::test_rel(r, expected, rel, "correlation");
    }

    {
        let pv = p_variance(slice_a, slice_b);
        let expected = 18.8421052631579;
        gsl::test_rel(pv, expected, rel, "p_variance");
    }

    {
        let t = t_test(slice_a, slice_b);
        let expected = -1.45701922702927;
        gsl::test_rel(t, expected, rel, "t_test");
    }

    {
        let (maxf, max_index) = max(slice_a);
        let max = maxf as i64;
        let expected = 22i64;
        let expected_index = 9;

        let str1 = format!("max ({} observed vs {} expected)", max, expected);
        gsl::test(max != expected, &str1);

        let str2 = format!("max index ({} observed vs {} expected)",
                           max_index,
                           expected_index);
        gsl::test(max_index != expected_index, &str2);
    }

    {
        let (minf, min_index) = min(slice_a);
        let min = minf as i32;
        let expected = 8;
        let expected_index = 12;

        let str1 = format!("min ({} observed vs {} expected)", min, expected);
        gsl::test(min != expected, &str1);

        let str2 = format!("min index ({} observed vs {} expected)",
                           min_index,
                           expected_index);
        gsl::test(min_index != expected_index, &str2);
    }

    {
        let (minf, min_index, maxf, max_index) = minmax(slice_a);
        let min = minf as i32;
        let max = maxf as i32;
        let expected_max = 22;
        let expected_min = 8;
        let expected_max_index = 9;
        let expected_min_index = 12;

        let str1 = format!("minmax max ({} observed vs {} expected)", max, expected_max);
        gsl::test(max != expected_max, &str1);

        let str2 = format!("minmax min ({} observed vs {} expected)", min, expected_min);
        gsl::test(min != expected_min, &str2);

        let str3 = format!("minmax index max ({} observed vs {} expected)",
                           max_index,
                           expected_max_index);
        gsl::test(max_index != expected_max_index, &str3);

        let str4 = format!("minmax index min ({} observed vs {} expected)",
                           min_index,
                           expected_min_index);
        gsl::test(min_index != expected_min_index, &str4);
    }

    let mut sorted = slice_a.clone();
    sorted.sort();

    {
        let median = median_from_sorted_data(&sorted);
        let expected = 18.0;
        gsl::test_rel(median, expected, rel, "median_from_sorted_data(even)");
    }

    {
        let zeroth = quantile_from_sorted_data(&sorted, 0.0);
        let expected = 8.0;
        gsl::test_rel(zeroth, expected, rel, "quantile_from_sorted_data(0)");
    }

    {
        let top = quantile_from_sorted_data(&sorted, 1.0);
        let expected = 22.0;
        gsl::test_rel(top, expected, rel, "quantile_from_sorted_data(100)");
    }

    {
        let median = quantile_from_sorted_data(&sorted, 0.5);
        let expected = 18.0;
        gsl::test_rel(median, expected, rel, "quantile_from_sorted_data(50, even)");
    }
}
