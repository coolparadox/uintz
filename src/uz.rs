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
        let (hi, hic) = self.hi.addc(self.hi.min_value(), loc);
        (Self { hi, lo }, hic)
    }

    fn augment(self) -> Uz<Self> {
        Uz {
            hi: self.min_value(),
            lo: self,
        }
    }

    fn max_value(self) -> Self {
        Self {
            hi: self.hi.max_value(),
            lo: self.hi.max_value(),
        }
    }

    fn min_value(self) -> Self {
        Self {
            hi: self.hi.min_value(),
            lo: self.hi.min_value(),
        }
    }
}
