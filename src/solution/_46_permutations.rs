

/// [Problem](https://leetcode-cn.com/problems//)
/// --------------------------------------------------------------------------
///
/// 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。
///
/// ```text
///
/// 示例 1：
///
/// 输入：nums = [1,2,3]
/// 输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
///
/// 示例 2：
///
/// 输入：nums = [0,1]
/// 输出：[[0,1],[1,0]]
///
///
/// 示例 3：
///
/// 输入：nums = [1]
/// 输出：[[1]]
///
///
/// 提示：
///
///
/// 1 <= nums.length <= 6
/// -10 <= nums[i] <= 10
/// nums 中的所有整数 互不相同
/// ```
///
/// Tips
/// -----
/// 回溯法：
///
///
///
/// Answer
/// ------

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &Vec<i32>, visited: &mut HashMap<i32, bool>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                ans.push(path.clone());
                return;
            }

            for &i in nums {
                match visited.contains_key(&i) {
                    true => continue,
                    false => {
                        visited.insert(i, true);
                        path.push(i);
                        backtracking(nums, visited, path, ans);
                        visited.remove(&i);
                        path.pop();
                    }
                }
            }
        }

        let mut ans = vec![];
        let mut path = vec![];
        let mut visited = HashMap::new();
        backtracking(&nums, &mut visited, &mut path, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![1], vec![vec![1]]),
            (vec![0, 1], vec![vec![0, 1], vec![1, 0]]),
            (vec![1, 2, 3], vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]),
        ];
        for (nums, answer) in test_case {
            let ans = Solution::permute(nums);
            assert_eq!(ans, answer);
        }
    }
}
