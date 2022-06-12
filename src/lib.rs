// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.

// section clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::print_stdout)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::let_underscore_drop)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::inline_always)]
#![allow(clippy::unwrap_in_result)]

use std::borrow::Borrow;
// section uses
use std::option::Option;

#[repr(u8)]
pub enum Tristate {
    True = 1,
    False = 0,
    Else = 2,
}

impl Tristate {
    pub fn is_true(&self) -> bool {
        self == Tristate::True
    }

    pub fn is_false(&self) -> bool {
        self == Tristate::False
    }

    pub fn is_other(&self) -> bool {
        self == Tristate::Else
    }

    pub fn is(&self, other: &Tristate) -> bool {
        self == other
    }
}

impl AsRef<Tristate> for Tristate {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Tristate> for Tristate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl From<bool> for Tristate {
    fn from(b: bool) -> Self {
        if b {
            Tristate::True
        } else {
            Tristate::False
        }
    }
}

impl Into<bool> for Tristate {
    fn into(self) -> bool {
        if self == Tristate::True {
            true
        } else {
            false
        }
    }
}

impl Into<Option<bool>> for Tristate {
    fn into(self) -> Option<bool> {
        if self == Tristate::True {
            Some(true)
        } else if self == Tristate::False {
            Some(false)
        } else {
            None
        }
    }
}

impl From<Option<bool>> for Tristate {
    fn from(b: Option<bool>) -> Self {
        if let Some(b) = b {
            if b {
                Tristate::True
            } else {
                Tristate::False
            }
        } else {
            Tristate::Else
        }
    }
}
