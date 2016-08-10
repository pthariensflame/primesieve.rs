extern crate libc;
use libc::{c_int, c_void, int64_t, size_t, uint64_t};

// This `const` is a `#define` in the original C header ("primesieve.h").
pub const PRIMESIEVE_ERROR: uint64_t = !(0 as uint64_t) as uint64_t;

// These `const`s are an anonymous enum in the original C header
// ("primesieve.h").
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
    // These `fn`s are from the C header "primesieve.h".
    pub fn primesieve_generate_primes(start: uint64_t,
                                      stop: uint64_t,
                                      size: *mut size_t,
                                      type_: c_int)
                                      -> *mut c_void;
    pub fn primesieve_generate_n_primes(n: uint64_t, start: uint64_t, type_: c_int) -> *mut c_void;
    pub fn primesieve_nth_prime(n: int64_t, start: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_nth_prime(n: int64_t, start: uint64_t) -> uint64_t;
    pub fn primesieve_count_primes(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_count_twins(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_count_triplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_count_quadruplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_count_quintuplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_count_sextuplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_primes(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_twins(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_triplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_quadruplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_quintuplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_parallel_count_sextuplets(start: uint64_t, stop: uint64_t) -> uint64_t;
    pub fn primesieve_print_primes(start: uint64_t, stop: uint64_t);
    pub fn primesieve_print_twins(start: uint64_t, stop: uint64_t);
    pub fn primesieve_print_triplets(start: uint64_t, stop: uint64_t);
    pub fn primesieve_print_quadruplets(start: uint64_t, stop: uint64_t);
    pub fn primesieve_print_quintuplets(start: uint64_t, stop: uint64_t);
    pub fn primesieve_print_sextuplets(start: uint64_t, stop: uint64_t);
    pub fn primesieve_callback_primes(start: uint64_t,
                                      stop: uint64_t,
                                      callback: extern "C" fn(prime: uint64_t));
}
