// gsl.rs
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

pub fn test(status: bool, test_description: &str) {
    if status == true {
        println!("FAIL: {}\n", test_description);
        assert!(false);
    }
}

pub fn test_rel(result: f64, expected: f64, relative_error: f64, test_description: &str) {
    // Check for NaN vs inf vs number
    let status = if result.is_nan() || expected.is_nan() {
        result.is_nan() != expected.is_nan()
    } else if result.is_infinite() || expected.is_infinite() {
        result.is_infinite() != expected.is_infinite()
    } else if expected != 0.0 {
        (result - expected).abs() / expected.abs() > relative_error
    } else {
        result.abs() > relative_error
    };

    if status == true {
        println!("FAIL: {} ({} observed vs {} expected)",
                 test_description,
                 result,
                 expected);
        assert!(false);
    }
}
