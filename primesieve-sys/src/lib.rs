extern crate libc;
use libc::{uint64_t, c_int, c_void, size_t};

// This `const` is a `#define` in the original C header.
pub const PRIMESIEVE_ERROR: uint64_t = !(0 as uint64_t) as uint64_t;

// These `const`s are an anonymous enum in the original C header.
pub const SHORT_PRIMES: c_int = 0;
pub const USHORT_PRIMES: c_int = 1;
pub const INT_PRIMES: c_int = 2;
pub const UINT_PRIMES: c_int = 3;
pub const LONG_PRIMES: c_int = 4;
pub const ULONG_PRIMES: c_int = 5;
pub const LONGLONG_PRIMES: c_int = 6;
pub const ULONGLONG_PRIMES: c_int = 7;
pub const INT16_PRIMES: c_int = 8;
pub const UINT16_PRIMES: c_int = 9;
pub const INT32_PRIMES: c_int = 10;
pub const UINT32_PRIMES: c_int = 11;
pub const INT64_PRIMES: c_int = 12;
pub const UINT64_PRIMES: c_int = 13;

#[link(name = "primesieve")]
extern "C" {
    pub fn primesieve_generate_primes(start: uint64_t,
                                      stop: uint64_t,
                                      size: *mut size_t,
                                      type_: c_int)
                                      -> *mut c_void;
}
