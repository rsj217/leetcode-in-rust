pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut dp = vec![0, 1];
        for _i in 2..=n {
            let dpn = dp[0] + dp[1];
            dp[0] = dp[1];
            dp[1] = dpn;
        }
        dp[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (10, 55),
        ];

        for (num, answer) in test_case {
            let ans = Solution::fib(num);
            assert_eq!(ans, answer);
        }
    }
}
