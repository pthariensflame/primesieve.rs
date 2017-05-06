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

use std::slice;

pub extern crate libc;

pub extern crate primesieve_sys as raw;

extern crate num_traits;
use num_traits::cast::cast as num_cast;

pub mod max_stop {
    pub fn get() -> u64 {
        unsafe { super::raw::primesieve_get_max_stop() }
    }
}

pub mod sieve_size {
    use super::num_cast;

    pub fn set<N: Into<u16>>(sieve_size: N) -> bool {
        if let Some(n_) = num_cast::<u16, super::libc::c_int>(sieve_size.into()) {
            if n_ >= 1 && n_ <= 2048 {
                unsafe {
                    super::raw::primesieve_set_sieve_size(n_);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get() -> u16 {
        num_cast::<super::libc::c_int, u16>(unsafe { super::raw::primesieve_get_sieve_size() })
            .unwrap_or_else(|| unreachable!())
    }
}

pub mod num_threads {
    use super::num_traits::cast::cast as num_cast;

    pub fn set<N: Into<Option<u64>>>(num_threads: N) -> bool {
        if let Some(n) = num_threads.into() {
            if let Some(n_) = num_cast::<u64, super::libc::c_int>(n) {
                if n_ >= 1 {
                    unsafe {
                        super::raw::primesieve_set_num_threads(n_);
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            unsafe {
                super::raw::primesieve_set_num_threads(-1);
            }
            true
        }
    }

    pub fn get() -> u64 {
        num_cast::<super::libc::c_int, u64>(unsafe { super::raw::primesieve_get_num_threads() })
            .unwrap_or_else(|| unreachable!())
    }
}


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
    fn default() -> Self {
        Tupling::One
    }
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
    fn to_tupling(self) -> Option<Tupling> {
        Some(self)
    }
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
            stop: max_stop::get(),
        }
    }

    pub fn tupling<T: ToTupling>(mut self, tupling: T) -> Option<Self> {
        if let Some(t) = tupling.to_tupling() {
            self.tupling = t;
            Some(self)
        } else {
            None
        }
    }

    pub fn is_parallel<B: Into<bool>>(mut self, is_parallel: B) -> Self {
        self.is_parallel = is_parallel.into();
        self
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = start.into();
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = stop.into();
        self
    }

    pub fn get(self) -> Option<u64> {
        let result = if self.is_parallel {
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
        };
        if result == raw::PRIMESIEVE_ERROR {
            Some(result)
        } else {
            None
        }
    }
}

impl Default for Count {
    fn default() -> Self {
        Count::new()
    }
}

impl From<Count> for u64 {
    fn from(v: Count) -> u64 {
        v.get().expect("primesieve error")
    }
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

    pub fn is_parallel<B: Into<bool>>(mut self, is_parallel: B) -> Self {
        self.is_parallel = is_parallel.into();
        self
    }

    pub fn after<N: Into<u64>>(mut self, n: N) -> Option<Self> {
        if let Some(n_) = num_cast::<u64, libc::int64_t>(n.into()) {
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

    pub fn before<N: Into<u64>>(mut self, n: N) -> Option<Self> {
        if let Some(n_) = num_cast::<u64, libc::int64_t>(n.into()) {
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

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = start.into();
        self
    }

    pub fn get(self) -> Option<u64> {
        let result = if self.is_parallel {
            unsafe { raw::primesieve_nth_prime(self.n, self.start) }
        } else {
            unsafe { raw::primesieve_parallel_nth_prime(self.n, self.start) }
        };
        if result == raw::PRIMESIEVE_ERROR {
            Some(result)
        } else {
            None
        }
    }
}

impl Default for Nth {
    fn default() -> Self {
        Nth::new()
    }
}

impl From<Nth> for u64 {
    fn from(v: Nth) -> u64 {
        v.get().expect("primesieve error")
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct Print {
    pub tupling: Tupling,
    pub start: u64,
    pub stop: u64,
}

impl Print {
    pub fn new() -> Self {
        Print {
            tupling: Tupling::One,
            start: 0,
            stop: max_stop::get(),
        }
    }

    pub fn tupling<T: ToTupling>(mut self, tupling: T) -> Option<Self> {
        if let Some(t) = tupling.to_tupling() {
            self.tupling = t;
            Some(self)
        } else {
            None
        }
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = start.into();
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = stop.into();
        self
    }

    pub fn execute(self) {
        match self.tupling {
            Tupling::One => unsafe { raw::primesieve_print_primes(self.start, self.stop) },
            Tupling::Two => unsafe { raw::primesieve_print_twins(self.start, self.stop) },
            Tupling::Three => unsafe { raw::primesieve_print_triplets(self.start, self.stop) },
            Tupling::Four => unsafe { raw::primesieve_print_quadruplets(self.start, self.stop) },
            Tupling::Five => unsafe { raw::primesieve_print_quintuplets(self.start, self.stop) },
            Tupling::Six => unsafe { raw::primesieve_print_sextuplets(self.start, self.stop) },
        }
    }
}

impl Default for Print {
    fn default() -> Self {
        Print::new()
    }
}

pub unsafe trait Generable: Clone {
    fn type_key() -> libc::c_int;
}

unsafe impl Generable for u16 {
    fn type_key() -> libc::c_int {
        raw::UINT16_PRIMES
    }
}

unsafe impl Generable for u32 {
    fn type_key() -> libc::c_int {
        raw::UINT32_PRIMES
    }
}

unsafe impl Generable for u64 {
    fn type_key() -> libc::c_int {
        raw::UINT64_PRIMES
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct Generate {
    pub start: u64,
    pub stop: u64,
}

impl Generate {
    pub fn new() -> Self {
        Generate {
            start: 0,
            stop: max_stop::get(),
        }
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = start.into();
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = stop.into();
        self
    }

    pub fn get<N: Generable>(self) -> Vec<N> {
        let mut size: libc::size_t = 0;
        let raw_arr = unsafe {
            raw::primesieve_generate_primes(self.start, self.stop, &mut size, N::type_key())
        };
        let result: Vec<N> = unsafe {
                slice::from_raw_parts(raw_arr as *mut N,
                                                       num_cast::<libc::size_t, usize>(size)
                                                           .expect("primesieve error"))
            }
            .to_owned();
        unsafe {
            raw::primesieve_free(raw_arr);
        }
        result
    }
}

impl Default for Generate {
    fn default() -> Self {
        Generate::new()
    }
}

impl<N: Generable> Into<Vec<N>> for Generate {
    fn into(self) -> Vec<N> {
        self.get()
    }
}

#[derive(Debug)]
pub struct Iter {
    raw_iter: *mut raw::primesieve_iterator,
}
