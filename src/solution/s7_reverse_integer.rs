pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let signed = x < 0;
        let abs_x = if signed {
            -1 * x
        } else {
            x
        };
        let mut ret: i64 = 0;
        let mut buf = abs_x;
        while buf > 0 {
            ret = ret * 10;
            let z = buf / 10;
            let y: i64 = (buf - z * 10) as i64;
            ret = ret + y;
            buf = z;
        }
        if signed {
            if ret > 2147483648 {
                0
            } else {
                -1 * ret as i32
            }
        } else {
            if ret > 2147483647 {
                0
            } else {
                ret as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(0, Solution::reverse(1534236469));
    }
}
