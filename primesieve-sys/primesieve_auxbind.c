// primesieve_auxbind.c
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

#include "primesieve.h"

uint64_t primesieve_next_prime_auxbind(primesieve_iterator *pi) {
  return primesieve_next_prime(pi);
}

uint64_t primesieve_prev_prime_auxbind(primesieve_iterator *pi) {
  return primesieve_prev_prime(pi);
}
