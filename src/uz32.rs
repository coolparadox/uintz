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
    fn min_value0() {
        assert_eq!(new(u32::max_value()).min_value(), new(0));
    }

    #[test]
    fn augment0() {
        let v = new(u32::max_value());
        let va = v.augment();
        assert_eq!(
            va,
            Uz {
                hi: v.min_value(),
                lo: v,
            }
        );
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
        let (v, c) = new(0).max_value().addc(new(1), false);
        assert_eq!(v, new(0));
        assert_eq!(c, true);
    }

    #[test]
    fn addc32_0() {
        let (v, c) = new(0).addc32(1, false);
        assert_eq!(v, new(1));
        assert_eq!(c, false);
    }

    #[test]
    fn addc32_1() {
        let (v, c) = new(0).addc32(1, true);
        assert_eq!(v, new(2));
        assert_eq!(c, false);
    }

    #[test]
    fn addc32_2() {
        let (v, c) = new(0).max_value().addc32(1, false);
        assert_eq!(v, new(0));
        assert_eq!(c, true);
    }

    #[test]
    fn mulc_0() {
        let (v, c) = new(1).mulc(new(1), new(0));
        assert_eq!(v, new(1));
        assert_eq!(c, new(0));
    }

    #[test]
    fn mulc_1() {
        let (v, c) = new(4).mulc(new(10), new(3));
        assert_eq!(v, new(43));
        assert_eq!(c, new(0));
    }

    #[test]
    fn mulc_2() {
        let (v, c) = new(0).max_value().mulc(new(1), new(123));
        assert_eq!(v, new(122));
        assert_eq!(c, new(1));
    }

    #[test]
    fn mulc32_0() {
        let (v, c) = new(1).mulc32(1, new(0));
        assert_eq!(v, new(1));
        assert_eq!(c, new(0));
    }

    #[test]
    fn mulc32_1() {
        let (v, c) = new(4).mulc32(10, new(3));
        assert_eq!(v, new(43));
        assert_eq!(c, new(0));
    }

    #[test]
    fn mulc32_2() {
        let (v, c) = new(0).max_value().mulc32(1, new(123));
        assert_eq!(v, new(122));
        assert_eq!(c, new(1));
    }

    #[test]
    fn subb0() {
        let (v, c) = new(1).subb(new(1), false);
        assert_eq!(v, new(0));
        assert_eq!(c, false);
    }

    #[test]
    fn subb1() {
        let (v, c) = new(0).subb(new(1), false);
        assert_eq!(v, new(0).max_value());
        assert_eq!(c, true);
    }

    #[test]
    fn subb2() {
        let (v, c) = new(1).subb(new(1), true);
        assert_eq!(v, new(0).max_value());
        assert_eq!(c, true);
    }

    #[test]
    fn subb32_0() {
        let (v, c) = new(1).subb32(1, false);
        assert_eq!(v, new(0));
        assert_eq!(c, false);
    }

    #[test]
    fn subb32_1() {
        let (v, c) = new(0).subb32(1, false);
        assert_eq!(v, new(0).max_value());
        assert_eq!(c, true);
    }

    #[test]
    fn subb32_2() {
        let (v, c) = new(1).subb32(1, true);
        assert_eq!(v, new(0).max_value());
        assert_eq!(c, true);
    }

}

impl Uintz for Uz32 {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        self.addc32(other.v, carry)
    }

    fn addc32(self, other: u32, carry: bool) -> (Self, bool) {
        let nv: u64 = self.v as u64 + other as u64 + if carry { 1 } else { 0 };
        (
            Self {
                v: (nv % 0x1_0000_0000u64) as u32,
            },
            nv / 0x1_0000_0000u64 != 0,
        )
    }

    fn augment(self) -> Uz<Self> {
        Uz {
            hi: self.min_value(),
            lo: self,
        }
    }

    fn max_value(self) -> Self {
        Self {
            v: u32::max_value(),
        }
    }

    fn min_value(self) -> Self {
        Self { v: 0 }
    }

    fn mulc32(self, other: u32, carry: Self) -> (Self, Self) {
        let nv: u64 = self.v as u64 * other as u64 + carry.v as u64;
        (
            Self {
                v: (nv % 0x1_0000_0000u64) as u32,
            },
            Self {
                v: (nv / 0x1_0000_0000u64) as u32,
            },
        )
    }

    fn mulc(self, other: Self, carry: Self) -> (Self, Self) {
        self.mulc32(other.v, carry)
    }

    fn subb(self, other: Self, borrow: bool) -> (Self, bool) {
        self.subb32(other.v, borrow)
    }

    fn subb32(self, other: u32, borrow: bool) -> (Self, bool) {
        let v = self.v as u64;
        let o = other as u64 + if borrow { 1 } else { 0 };
        let nb = o > v;
        let nv = if nb { v + 0x1_0000_0000u64 } else { v } - o;
        (Self { v: nv as u32 }, nb)
    }
}
