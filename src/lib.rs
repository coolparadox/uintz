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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz32 {
    v: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz<T: Uintz> {
    hi: T,
    lo: T,
}

pub trait Uintz {
    fn zero(&self) -> Self;

    fn augment(self) -> Uz<Self>
    where
        Self: std::marker::Sized;

    fn addc(self, other: Self, carry: bool) -> (Self, bool)
    where
        Self: std::marker::Sized;
}
