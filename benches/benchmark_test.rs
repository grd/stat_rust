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

#![feature(test)]

extern crate test;
extern crate stat;

use test::Bencher;
use stat::*;

static DATA0: [f64; 1] = [1.0];
static DATA1: [f64; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
static DATA2: [f64; 10000] = [0.0; 10000];

static DATA0_I: [i32; 1] = [1];
static DATA1_I: [i32; 6] = [0, 1, 2, 3, 4, 5];
static DATA2_I: [i32; 10000] = [0; 10000];

#[bench]
fn bench_mean0(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA0[..]));
}

#[bench]
fn bench_mean1(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA1[..]));
}

#[bench]
fn bench_mean2(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA2[..]));
}

#[bench]
fn bench_mean0_i(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA0_I[..]));
}

#[bench]
fn bench_mean1_i(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA1_I[..]));
}

#[bench]
fn bench_mean2_i(b: &mut Bencher) {
    b.iter(|| stat::mean(&DATA2_I[..]));
}
