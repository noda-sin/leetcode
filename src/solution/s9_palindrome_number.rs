pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut p: i64 = 0;
        let mut buf = x;
        while buf > 0 {
            p = p * 10;
            let z = buf / 10;
            let y: i64 = (buf - z * 10) as i64;
            p = p + y;
            buf = z;
        }

        let a: i64 = x as i64;
        a == p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-101));
    }
}
