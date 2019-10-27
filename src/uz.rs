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

use super::Uintz;
use super::Uz32;
use super::Uz;

impl Uintz for Uz<Uz32> {

    fn zero(&self) -> Self {
        Self {
            hi: self.hi.zero(),
            lo: self.hi.zero(),
        }
    }

    fn augment(self) -> Uz<Self> {
        Uz { hi: self.zero(), lo: self, }
    }

    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc(other.lo, carry);
        let (hi, hic) = self.hi.addc(other.hi, loc);
        (Self { hi, lo }, hic)
    }

}
