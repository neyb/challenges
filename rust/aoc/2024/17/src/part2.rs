use anyhow::*;

type Res = u64;

/**
* the algo is the following: 2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0
*
* ``` none
* 2,4 -> bst 4 -> B = A%8         B only depends on A
* 1,1 -> bxl 1 -> B = B⊻1
* 7,5 -> cdv 5 -> C = A / (2^B)   because B only depends on A, C only depends on A
* 1,5 -> bxl 5 -> B = B⊻5
* 4,0 -> bxc 0 -> B = B⊻C         because B and C only depend on A, B only depends on A
* 0,3 -> adv 3 -> A = A/8
* 5,5 -> out 5 -> out B
* 3,0 -> jmp 0 -> until A = 0
* ```
*
* also (if initial a != 0) :
* ``` none
* while a != 0 {
*   b = some_function(a)
*   //                ^ notice b only depends on a
*   print b
*   a >>= 3
*   // this shift means we only rely on higher weight bits on next iteration
* }
* ```
*
* we can start by guessing higher weight bits (3 by 3)
* and try to find the program in reverse order with a dfs
*
* because in the end, a is 0 (to break the loop),
* we can start by guessing the last value of a is 0
*/
pub(crate) fn run() -> Result<Res> {
    let mut desired_output = [2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0];
    desired_output.reverse();
    rec_resolve(0, &desired_output).ok_or_else(|| anyhow!("no solution found"))
}

fn rec_resolve(guess: Res, desired_output: &[u8]) -> Option<Res> {
    let Some(next) = desired_output.first() else {
        return Some(guess);
    };

    (0..8)
        .map(|x| (guess << 3) | x)
        .filter(|&a| &next_b_output(a) == next)
        .filter_map(|a| rec_resolve(a, &desired_output[1..]))
        .next()
}

fn next_b_output(a: u64) -> u8 {
    let mut b;
    b = a & 7; // 2,4 -> bst 4 -> B = A%8 = A & 7
    b ^= 1; // 1,1 -> bxl 1 -> B = B ⊻ 1
    let c = a >> b; // 7,5 -> cdv 5 -> C = A / (2^B) = A >> B
    b ^= 5; // 1,5 -> bxl 5 -> B = B⊻5
    b ^= c; // 4,0 -> bxc 0 -> B = B⊻C
    (b & 7) as u8 // the output is the last 3 bits of B
}
