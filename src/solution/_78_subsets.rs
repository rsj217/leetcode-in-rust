use rand::Rng;

/// [Problem](https://leetcode-cn.com/problems/subsets/)
/// --------------------------------------------------------------------------
///
/// 给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
///
/// 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
///
///
/// ```text
/// 示例 1：
///
/// 输入：nums = [1,2,3]
/// 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
///
/// 示例 2：
///
/// 输入：nums = [0]
/// 输出：[[],[0]]
///
/// 提示：
///
/// 1 <= nums.length <= 10
/// -10 <= nums[i] <= 10
/// nums 中的所有元素 互不相同
/// ```
///
/// Tips
/// -----
/// * DFS 回溯法：
/// * BFS
///
/// Answer
/// ------

pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let num = rand::thread_rng().gen_range(0..2);
        match num {
            0 => Solution::dfs(nums),
            1 => Solution::bfs(nums),
            _ => panic!("num err"),
        }
    }


    pub fn dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &Vec<i32>, index: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if nums.len() < index {
                return;
            }
            ans.push(path.clone());
            for i in index..nums.len() {
                path.push(nums[i]);
                backtracking(nums, i + 1, path, ans);
                path.pop();
            }
        }
        let mut ans = vec![];
        let mut path = vec![];
        backtracking(&nums, 0, &mut path, &mut ans);
        ans
    }


    pub fn bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];
        for num in nums {
            let size = ans.len();
            for i in 0..size {
                let mut path = ans[i].clone();
                path.push(num);
                ans.push(path);
            }
        }
        ans
    }
}

fn equal(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> bool {
    let (mut nums1, mut nums2) = (nums1, nums2);
    nums1.sort();
    nums2.sort();
    nums1 == nums2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![1, 3], vec![vec![], vec![1], vec![1, 3], vec![3]]),
            (vec![1, 2, 3], vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]]),
            (vec![1, 5, 3], vec![vec![], vec![1], vec![1, 5], vec![1, 5, 3], vec![1, 3], vec![5], vec![5, 3], vec![3]]),
        ];
        for (nums, answer) in test_case {
            let ans = Solution::subsets(nums);
            assert!(equal(ans, answer));
        }
    }
}
