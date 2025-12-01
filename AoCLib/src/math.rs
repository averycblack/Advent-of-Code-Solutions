use std::mem::swap;

// Binary GCD:
// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
#[allow(dead_code)]
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    }

    if v == 0 {
        return u;
    }

    let gcd_binary_exponent = (u | v).trailing_zeros();
    while u != v {
        if u < v {
            swap(&mut u, &mut v);
        }

        u -= v;
        u >>= u.trailing_zeros();
    }

    u << gcd_binary_exponent
}

#[allow(dead_code)]
pub fn lcm(values: Vec<u64>) -> u64 {
    values
        .iter()
        .fold(1, |lcm, factor| lcm * factor / gcd(lcm, *factor))
}
