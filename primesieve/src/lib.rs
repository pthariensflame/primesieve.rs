// lib.rs
// Copyright 2016 Alexander Altman
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

pub extern crate libc;

pub extern crate primesieve_sys as raw;

extern crate snowflake;
use snowflake::ProcessUniqueId;

extern crate num_traits;
use num_traits::cast::cast as num_cast;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash,Clone,Copy)]
pub enum Tupling {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Default for Tupling {
    fn default() -> Self { Tupling::One }
}

macro_rules! from_tupling_impl {
    ($t:ty) => {
        impl ::std::convert::From<$crate::Tupling> for $t {
            fn from(v: $crate::Tupling) -> $t {
                v as $t
            }
        }
    }
}

from_tupling_impl!(u8);
from_tupling_impl!(i8);
from_tupling_impl!(u16);
from_tupling_impl!(i16);
from_tupling_impl!(u32);
from_tupling_impl!(i32);
from_tupling_impl!(u64);
from_tupling_impl!(i64);
from_tupling_impl!(usize);
from_tupling_impl!(isize);

pub trait ToTupling {
    fn to_tupling(self) -> Option<Tupling>;
}

impl ToTupling for Tupling {
    fn to_tupling(self) -> Option<Tupling> { Some(self) }
}

macro_rules! to_tupling_impl {
    ($t:ty) => {
        impl $crate::ToTupling for $t {
            fn to_tupling(self) -> ::std::option::Option<$crate::Tupling> {
                if self == 1 {
                    ::std::option::Option::Some($crate::Tupling::One)
                } else if self == 2 {
                    ::std::option::Option::Some($crate::Tupling::Two)
                } else if self == 3 {
                    ::std::option::Option::Some($crate::Tupling::Three)
                } else if self == 4 {
                    ::std::option::Option::Some($crate::Tupling::Four)
                } else if self == 5 {
                    ::std::option::Option::Some($crate::Tupling::Five)
                } else if self == 6 {
                    ::std::option::Option::Some($crate::Tupling::Six)
                } else {
                    ::std::option::Option::None
                }
            }
        }
    }
}

to_tupling_impl!(u8);
to_tupling_impl!(i8);
to_tupling_impl!(u16);
to_tupling_impl!(i16);
to_tupling_impl!(u32);
to_tupling_impl!(i32);
to_tupling_impl!(u64);
to_tupling_impl!(i64);
to_tupling_impl!(usize);
to_tupling_impl!(isize);

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct Count {
    pub tupling: Tupling,
    pub is_parallel: bool,
    pub start: u64,
    pub stop: u64,
}

impl Count {
    pub fn new() -> Self {
        Count {
            tupling: Tupling::One,
            is_parallel: false,
            start: 0,
            stop: unsafe { raw::primesieve_get_max_stop() },
        }
    }

    pub fn tupling<T: ToOwned>(mut self, tupling: T) -> Option<Self>
        where T::Owned: ToTupling {
        if let Some(t) = tupling.to_owned().to_tupling() {
            self.tupling = t;
            Some(self)
        } else {
            None
        }
    }

    pub fn is_parallel<B: ToOwned>(mut self, is_parallel: B) -> Self
        where B::Owned: Into<bool> {
        self.is_parallel = is_parallel.to_owned().into();
        self
    }

    pub fn start<N: ToOwned>(mut self, start: N) -> Self
        where N::Owned: Into<u64> {
        self.start = start.to_owned().into();
        self
    }

    pub fn stop<N: ToOwned>(mut self, stop: N) -> Self
        where N::Owned: Into<u64> {
        self.stop = stop.to_owned().into();
        self
    }

    pub fn get(self) -> u64 {
        if self.is_parallel {
            match self.tupling {
                Tupling::One => unsafe {
                    raw::primesieve_parallel_count_primes(self.start, self.stop)
                },
                Tupling::Two => unsafe {
                    raw::primesieve_parallel_count_twins(self.start, self.stop)
                },
                Tupling::Three => unsafe {
                    raw::primesieve_parallel_count_triplets(self.start, self.stop)
                },
                Tupling::Four => unsafe {
                    raw::primesieve_parallel_count_quadruplets(self.start, self.stop)
                },
                Tupling::Five => unsafe {
                    raw::primesieve_parallel_count_quintuplets(self.start, self.stop)
                },
                Tupling::Six => unsafe {
                    raw::primesieve_parallel_count_sextuplets(self.start, self.stop)
                },
            }
        } else {
            match self.tupling {
                Tupling::One => unsafe { raw::primesieve_count_primes(self.start, self.stop) },
                Tupling::Two => unsafe { raw::primesieve_count_twins(self.start, self.stop) },
                Tupling::Three => unsafe { raw::primesieve_count_triplets(self.start, self.stop) },
                Tupling::Four => unsafe {
                    raw::primesieve_count_quadruplets(self.start, self.stop)
                },
                Tupling::Five => unsafe {
                    raw::primesieve_count_quintuplets(self.start, self.stop)
                },
                Tupling::Six => unsafe { raw::primesieve_count_sextuplets(self.start, self.stop) },
            }
        }
    }
}

impl Default for Count {
    fn default() -> Self { Count::new() }
}

impl From<Count> for u64 {
    fn from(v: Count) -> u64 { v.get() }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct Nth {
    pub is_parallel: bool,
    pub n: i64,
    pub start: u64,
}

impl Nth {
    pub fn new() -> Self {
        Nth {
            is_parallel: false,
            n: 0,
            start: 0,
        }
    }

    pub fn is_parallel<B: ToOwned>(mut self, is_parallel: B) -> Self
        where B::Owned: Into<bool> {
        self.is_parallel = is_parallel.to_owned().into();
        self
    }

    pub fn after<N: ToOwned>(mut self, n: N) -> Option<Self>
        where N::Owned: Into<u64> {
        if let Some(n_) = num_cast::<u64, libc::int64_t>(n.to_owned().into()) {
            if n_ >= 0 {
                self.n = -n_;
                Some(self)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn before<N: ToOwned>(mut self, n: N) -> Option<Self>
        where N::Owned: Into<u64> {
        if let Some(n_) = num_cast::<u64, libc::int64_t>(n.to_owned().into()) {
            if n_ > 0 {
                self.n = -n_;
                Some(self)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn start<N: ToOwned>(mut self, start: N) -> Self
        where N::Owned: Into<u64> {
        self.start = start.to_owned().into();
        self
    }

    pub fn get(self) -> u64 {
        if self.is_parallel {
            unsafe { raw::primesieve_nth_prime(self.n, self.start) }
        } else {
            unsafe { raw::primesieve_parallel_nth_prime(self.n, self.start) }
        }
    }
}

impl Default for Nth {
    fn default() -> Self { Nth::new() }
}

impl From<Nth> for u64 {
    fn from(v: Nth) -> u64 { v.get() }
}

#[derive(Debug)]
pub struct Iter {
    raw_iter: *mut raw::primesieve_iterator,
}
