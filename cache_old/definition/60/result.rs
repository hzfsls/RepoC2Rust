pub static set_num_primes: Global<u32> = global!((set_primes.len() / std::mem::size_of::<i32>()) as u32);
