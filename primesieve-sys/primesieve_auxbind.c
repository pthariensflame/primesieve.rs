#include "primesieve.h"

uint64_t primesieve_next_prime_auxbind(primesieve_iterator *pi) {
  return primesieve_next_prime(pi);
}

uint64_t primesieve_previous_prime_auxbind(primesieve_iterator *pi) {
  return primesieve_previous_prime(pi);
}
