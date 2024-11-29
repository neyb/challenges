use anyhow::*;

type Res = u64;
pub(crate) fn run(_content: &String) -> Result<Res> {
    // cf graph.png
    // rx receive low
    // when kj sources (dr, vn, ln, rx) send high
    // when             qs, pr, jv, jm  send low
    // when they receive high

    // period
    // for qs : 111100010110 = 2^1 + 2^2 + 2^4 + 2^8 + 2^9 + 2^10 + 2^11 = 3862
    // for pr : 111101100110 = 2^1 + 2^2 + 2^5 + 2^6 + 2^8 + 2^9 + 2^10 + 2^11 = 3942
    // for jv : 111110100010 = 2^1 + 2^5 + 2^7 + 2^8 + 2^9 + 2^10 + 2^11 = 4002
    // for jm : 111110010100 = 2^2 + 2^4 + 2^7 + 2^8 + 2^9 + 2^10 + 2^11 = 3988

    // we need to add 1 to each period to restore the original state

    let lcm1 = challenges_common::math::lcm(0b111100010110 + 1, 0b111101100110 + 1);
    let lcm2 = challenges_common::math::lcm(0b111110100010 + 1, 0b111110010100 + 1);
    let lcm = challenges_common::math::lcm(lcm1, lcm2);

    Ok(lcm)
}
