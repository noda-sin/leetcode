pub struct Solution {}

impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        if x >= y {
            return x - y;
        }

        if y % 2 == 0 {
            return 1 + Solution::broken_calc(x, y / 2);
        }

        return 1 + Solution::broken_calc(x, y + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::broken_calc(2, 3));
        assert_eq!(2, Solution::broken_calc(5, 8));
        assert_eq!(3, Solution::broken_calc(3, 10));
        assert_eq!(1023, Solution::broken_calc(1024, 1));
        assert_eq!(39, Solution::broken_calc(1, 1000000000));
    }
}
