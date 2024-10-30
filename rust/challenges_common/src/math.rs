use num_traits::Num;

pub fn gcd<N: Num + Copy + Ord>(a: N, b: N) -> N {
    if a < b {
        return gcd(b, a);
    }

    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<N: Num + Copy + Ord>(a: N, b: N) -> N {
    a / gcd(a, b) * b
}
