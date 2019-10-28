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

use crate::Uintz;
use crate::Uz;
use crate::Uz32;

#[cfg(test)]
mod tests {

    use crate::*;

    fn new(v: u32) -> Uz32 {
        from_u32(v)
    }

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
    fn zero0() {
        assert_eq!(new(u32::max_value()).zero(), new(0));
    }

    #[test]
    fn augment0() {
        let v = new(u32::max_value());
        let va = v.augment();
        assert_eq!(va, Uz { hi:v.zero(), lo:v, });
    }

    #[test]
    fn addc0() {
        let (v, c) = new(0).addc(new(1), false);
        assert_eq!(v, new(1));
        assert_eq!(c, false);
    }

    #[test]
    fn addc1() {
        let (v, c) = new(0).addc(new(1), true);
        assert_eq!(v, new(2));
        assert_eq!(c, false);
    }

    #[test]
    fn addc2() {
        let (v, c) = new(u32::max_value()).addc(new(1), false);
        assert_eq!(v, new(0));
        assert_eq!(c, true);
    }

}

impl Uintz for Uz32 {

    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let nv: u64 = self.v as u64 + other.v as u64 + if carry { 1 } else { 0 };
        (
            Self {
                v: (nv % 0x100000000u64) as u32,
            },
            nv / 0x100000000u64 != 0,
        )
    }

    fn augment(self) -> Uz<Self> {
        Uz {
            hi: self.zero(),
            lo: self,
        }
    }

    fn zero(&self) -> Self {
        Self {
            v: 0,
        }
    }

}
