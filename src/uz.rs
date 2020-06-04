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

impl Uintz for Uz<Uz32> {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc(other.lo, carry);
        let (hi, hic) = self.hi.addc(other.hi, loc);
        (Self { hi, lo }, hic)
    }

    fn addc32(self, other: u32, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc32(other, carry);
        let (hi, hic) = self.hi.addc32(0, loc);
        (Self { hi, lo }, hic)
    }

    fn augment(self) -> Uz<Self> {
        Uz {
            hi: self.zero(),
            lo: self,
        }
    }

    fn divr32(self, divisor: u32) -> (Self, u32) {
        (self, 0)
    }

    fn max_value(self) -> Self {
        Self {
            hi: self.hi.max_value(),
            lo: self.hi.max_value(),
        }
    }

    fn mulc(self, other: Self, carry: Self) -> (Self, Self) {
        let (ll, llc) = self.lo.mulc(other.lo, carry.lo);
        let (hl, hlc) = self.hi.mulc(other.lo, llc);
        let (lh, lhc) = self.lo.mulc(other.hi, hl);
        let (hh, hhc) = self.hi.mulc(other.hi, hlc);
        let (ah, ahc) = lh.addc(carry.hi, false);
        let (ac, acc) = lhc.addc(hh, ahc);
        let (aa, _) = hhc.addc32(0, acc);
        (Self { hi: ah, lo: ll }, Self { hi: aa, lo: ac })
    }

    fn mulc32(self, other: u32, carry: Self) -> (Self, Self) {
        let (lo, loc) = self.lo.mulc32(other, carry.lo);
        let (hi, hic) = self.hi.mulc32(other, loc);
        (
            Self { hi, lo },
            Self {
                hi: hic.zero(),
                lo: hic,
            },
        )
    }

    fn subb(self, other: Self, borrow: bool) -> (Self, bool) {
        let (lo, lob) = self.lo.subb(other.lo, borrow);
        let (hi, hib) = self.hi.subb(other.hi, lob);
        (Self { lo, hi }, hib)
    }

    fn subb32(self, other: u32, borrow: bool) -> (Self, bool) {
        let (lo, lob) = self.lo.subb32(other, borrow);
        let (hi, hib) = self.hi.subb32(0, lob);
        (Self { lo, hi }, hib)
    }

    fn zero(self) -> Self {
        Self {
            hi: self.hi.zero(),
            lo: self.hi.zero(),
        }
    }
}
