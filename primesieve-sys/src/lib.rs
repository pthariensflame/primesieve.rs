extern crate libc;
use libc::{c_char, c_int, c_void, int64_t, size_t, uint64_t};

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

// This `struct` is from the C header "primesieve/primesieve_iterator.h".
#[no_mangle]
#[repr(C)]
pub struct primesieve_iterator {
    pub i_: size_t,
    pub last_idx_: size_t,
    pub primes_: *mut uint64_t,
    pub primes_pimpl_: *mut uint64_t,
    pub start_: uint64_t,
    pub stop_: uint64_t,
    pub stop_hint_: uint64_t,
    pub tiny_cache_size_: uint64_t,
    pub is_error_: c_int,
}

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
    pub fn primesieve_get_sieve_size() -> c_int;
    pub fn primesieve_get_num_threads() -> c_int;
    pub fn primesieve_get_max_stop() -> uint64_t;
    pub fn primesieve_set_sieve_size(sieve_size: c_int);
    pub fn primesieve_set_num_threads(num_threads: c_int);
    pub fn primesieve_free(primes: *mut c_void);
    pub fn primesieve_test() -> c_int;
    pub fn primesieve_version() -> *const c_char;

    // These `fn`s are from the C header "primesieve/primesieve_iterator.h".
    pub fn primesieve_init(pi: *mut primesieve_iterator);
    pub fn primesieve_free_iterator(pi: *mut primesieve_iterator);
    pub fn primesieve_skipto(pi: *mut primesieve_iterator, start: uint64_t, stop_hint: uint64_t);
}

// These `fn`s are from the C header "primesieve/primesieve_iterator.h", but
// are declared as `static inline`, so we must bind to them in a roundabout way.
#[inline]
pub unsafe fn primesieve_next_prime(pi: *mut primesieve_iterator) -> uint64_t {
    primesieve_next_prime_auxbind(pi)
}
#[inline]
pub unsafe fn primesieve_previous_prime(pi: *mut primesieve_iterator) -> uint64_t {
    primesieve_previous_prime_auxbind(pi)
}
#[link(name = "primesieve_auxbind")]
extern "C" {
    fn primesieve_next_prime_auxbind(pi: *mut primesieve_iterator) -> uint64_t;
    fn primesieve_previous_prime_auxbind(pi: *mut primesieve_iterator) -> uint64_t;
}

#[cfg(test)]
mod test {
    use super::*;
    use super::libc::{c_int, uint64_t};
    use std::mem;

    #[test]
    fn builtin_test() {
        assert_eq!(unsafe { primesieve_test() }, 1 as c_int);
    }

    #[test]
    fn iterator_test() {
        let mut pi: primesieve_iterator = unsafe { mem::zeroed() };
        unsafe {
            primesieve_init(&mut pi);
            primesieve_skipto(&mut pi, 1 as uint64_t, 10 as uint64_t)
        }
        let x: uint64_t = unsafe { primesieve_next_prime(&mut pi) };
        unsafe {
            primesieve_skipto(&mut pi, 10 as uint64_t, 1 as uint64_t);
        }
        let y: uint64_t = unsafe { primesieve_previous_prime(&mut pi) };
        unsafe {
            primesieve_free_iterator(&mut pi);
        }
        assert_eq!(x, 2 as uint64_t);
        assert_eq!(y, 7 as uint64_t);
    }
}
