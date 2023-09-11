struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut curr = num;
        let mut number_of_steps = 0;

        while curr != 0 {
            if curr % 2 == 0 {
                curr /= 2;
            } else {
                curr -= 1;
            }
            number_of_steps += 1;
        }

        number_of_steps
    }
}
