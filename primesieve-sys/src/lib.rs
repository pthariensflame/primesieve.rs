extern crate libc;

pub const PRIMESIEVE_ERROR: libc::uint64_t = !(0 as libc::uint64_t) as libc::uint64_t;

#[no_mangle]
#[repr(C)]
pub enum PrimesType {
    SHORT_PRIMES,
    USHORT_PRIMES,
    INT_PRIMES,
    UINT_PRIMES,
    LONG_PRIMES,
    ULONG_PRIMES,
    LONGLONG_PRIMES,
    ULONGLONG_PRIMES,
    INT16_PRIMES,
    UINT16_PRIMES,
    INT32_PRIMES,
    UINT32_PRIMES,
    INT64_PRIMES,
    UINT64_PRIMES,
}

extern "C" {}
