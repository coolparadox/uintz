/*
 * Copyright 2019 Rafael Lorandi <coolparadox@gmail.com>
 *
 * This file is part of uintz, an arbitrary depth unsigned integer
 * library for the rust language.
 *
 * uintz is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * uintz is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with uintz.  If not, see <http://www.gnu.org/licenses/>
 */

use intz_utrait::Uintz;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eq0() {
        assert_eq!(new(0), new(0));
    }

    #[test]
    fn eq1() {
        assert_ne!(new(0), new(1));
    }

    #[test]
    fn eq2() {
        assert_eq!(new(u32::max_value()), new(u32::max_value()));
    }

    #[test]
    fn ord0() {
        assert!(new(0) < new(1));
    }

    #[test]
    fn ord1() {
        assert!(new(0) < new(u32::max_value()));
    }

    #[test]
    fn add0() {
        assert_eq!(new(0) + new(0), new(0))
    }

    #[test]
    fn add1() {
        assert_eq!(new(0) + new(1), new(1))
    }

    #[test]
    fn add2() {
        assert_eq!(new(1) + new(2), new(3))
    }

    #[test]
    fn add3() {
        assert_eq!(new(u32::max_value()) + new(0), new(u32::max_value()))
    }

    #[test]
    #[should_panic(expected = "integer overflow")]
    fn add4() {
        let _ = new(u32::max_value()) + new(1);
    }

    // FIXME: u32 methods at https://doc.rust-lang.org/std/primitive.u32.html

}

pub fn new(v: u32) -> Uz32 {
    Uz32 { v }
}

impl Uintz for Uz32 {
    fn zero(&self) -> Self {
        Self {
            v: 0,
        }
    }

    fn augment(self) -> Uz<Self> {
        Uz {
            hi: self.zero(),
            lo: self,
        }
    }

    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let nv: u64 = self.v as u64 + other.v as u64 + if carry { 1 } else { 0 };
        (
            Self {
                v: (nv % 0x100000000u64) as u32,
            },
            nv / 0x100000000u64 != 0,
        )
    }
}
