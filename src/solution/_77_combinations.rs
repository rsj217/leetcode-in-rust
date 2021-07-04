
/// [Problem](https://leetcode-cn.com/problems/combinations/)
/// --------------------------------------------------------------------------
///
/// 给定两个整数 n 和 k，返回 1 ... n 中所有可能的 k 个数的组合。
///
/// ```text
/// 示例:
///
/// 输入: n = 4, k = 2
///
/// 输出:
///
/// [
///   [2,4],
///   [3,4],
///   [2,3],
///   [1,2],
///   [1,3],
///   [1,4],
/// ]
/// ```
/// Tips
/// -----
/// 回溯法：
///
///
///
/// Answer
/// ------
pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtracking(n: i32, k: i32, index: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == k as usize {
                ans.push(path.clone());
                return;
            }
            for i in index..n + 1 {
                path.push(i);
                backtracking(n, k, i + 1, path, ans);
                path.pop();
            }
        }
        let mut ans = vec![];
        let mut path = vec![];
        backtracking(n, k, 1, &mut path, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (1, 1, vec![vec![1]]),
            (4, 2, vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]),
        ];
        for (n, k, answer) in test_case {
            let ans = Solution::combine(n, k);
            assert_eq!(ans, answer);
        }
    }
}
