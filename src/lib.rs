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

//! Quantii-types is a collection of special
//! types used by Quantii and related projects.

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
#![allow(clippy::exhaustive_enums)]

#[cfg(test)]
pub mod tests {
    #![allow(clippy::missing_panics_doc)]

    use super::*;
    #[test]
    pub fn true_is_true_1() {
        assert!(Tristate::True.is_true());
    }

    #[test]
    pub fn true_is_true_2() {
        assert!(!Tristate::True.is_false());
    }

    #[test]
    pub fn true_is_true_3() {
        assert!(Tristate::True.is(Tristate::True));
    }

    //noinspection RsAssertEqual
    #[test]
    pub fn true_is_true_4() {
        assert!(Tristate::True == Tristate::True);
    }

    #[test]
    pub fn false_is_false_1() {
        assert!(Tristate::False.is_false());
    }

    #[test]
    pub fn false_is_false_2() {
        assert!(!Tristate::False.is_true());
    }

    #[test]
    pub fn false_is_false_3() {
        assert!(Tristate::False.is(Tristate::False));
    }

    //noinspection RsAssertEqual
    #[test]
    pub fn false_is_false_4() {
        assert!(Tristate::False == Tristate::False);
    }

    #[test]
    pub fn else_is_else_1() {
        assert!(Tristate::Else.is_other());
    }

    #[test]
    pub fn else_is_else_2() {
        assert!(!Tristate::Else.is_true());
    }

    #[test]
    pub fn else_is_else_3() {
        assert!(!Tristate::Else.is_false());
    }

    #[test]
    pub fn else_is_else_4() {
        assert!(Tristate::Else.is(Tristate::Else));
    }

    //noinspection RsAssertEqual
    #[test]
    pub fn else_is_else_5() {
        assert!(Tristate::Else == Tristate::Else);
    }
}

use std::borrow::Borrow;
// section uses
use std::option::Option;

/// A tri-state boolean.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tristate {
    /// boolean `true`
    True = 1,
    /// boolean `false`
    False = 0,
    /// Undefined, other, or any other third value.
    Else = 2,
}

impl Tristate {
    /// Returns whether or not it is true
    #[must_use]
    pub fn is_true(self) -> bool {
        self == Self::True
    }

    /// Returns whether or not it is false
    #[must_use]
    pub fn is_false(self) -> bool {
        self == Self::False
    }

    /// Returns whether or not it is a different value
    #[must_use]
    pub fn is_other(self) -> bool {
        self == Self::Else
    }

    /// Returns whether or not it is equivalent to the parameter `other`
    #[must_use]
    pub fn is(self, other: Self) -> bool {
        self == other
    }
}

impl AsRef<Self> for Tristate {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for Tristate {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl From<bool> for Tristate {
    fn from(b: bool) -> Self {
        if b {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<Tristate> for bool {
    fn from(old: Tristate) -> Self {
        old == Tristate::True
    }
}

impl From<Tristate> for Option<bool> {
    fn from(old: Tristate) -> Self {
        if old == Tristate::True {
            Some(true)
        } else if old == Tristate::False {
            Some(false)
        } else {
            None
        }
    }
}

impl From<Option<bool>> for Tristate {
    fn from(old: Option<bool>) -> Self {
        old.map_or(
            Self::Else,
            |old_as| if old_as { Self::True } else { Self::False },
        )
    }
}

/// A tree that can have more than 2 children.
#[derive(Debug, PartialEq, Eq)]
pub struct NonBinaryTree<'children, T> {
    pub value: T,
    pub children: &'children [&'children NonBinaryTree<'children, T>],
}

impl<'children, T> NonBinaryTree<'children, T> {
    /// Create a new tree without an children
    pub const fn new(value: T) -> Self {
        Self {
            value,
            children: &[],
        }
    }

    /// Create a new tree with an arbitrary number of children
    pub const fn new_with_children(
        value: T,
        children: &'children [&'children NonBinaryTree<'children, T>],
    ) -> Self {
        Self { value, children }
    }
}
