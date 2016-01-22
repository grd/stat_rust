// types.rs
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

pub trait F64 {
    fn f64(&self) -> f64;
}

impl F64 for i8 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for i16 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for i32 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for i64 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for u8 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for u16 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for u32 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for u64 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}
impl F64 for isize {
    fn f64(&self) -> f64 {
        *self as f64
    }
}
impl F64 for usize {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for f32 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}

impl F64 for f64 {
    fn f64(&self) -> f64 {
        *self as f64
    }
}
