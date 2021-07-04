
/// [Problem](https://leetcode-cn.com/problems/two-sum/)
/// ---------------------------------------------------------------------------------
///
/// 定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 的那 两个 整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 你可以按任意顺序返回答案。
///
/// ```text
/// 示例 1：
///
/// 入：nums = [2,7,11,15], target = 9
/// 出：[0,1]
/// 释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
///
/// 示例 2：
///
/// 入：nums = [3,2,4], target = 6
/// 出：[1,2]
///
/// 示例 3：
///
/// 入：nums = [3,3], target = 6
/// 出：[0,1]
///
///
/// 提示：
///
/// 2 <= nums.length <= 103
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 只会存在一个有效答案
/// ```
///
/// Tips
/// -----
///
/// 图片 ![Spanish](../../../../img/Spanish.jpg)
///
/// Answer
/// -------


pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut t = std::collections::HashMap::with_capacity(nums.len());
        for (index, &item) in nums.iter().enumerate() {
            let index = index as i32;
            match t.get(&(target - item)) {
                Some(&i) => return vec![i, index as i32],
                None => t.insert(item, index),
            };
        }
        return vec![];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution(){
        let test_case = vec![
            (vec![2, 7, 1, 5], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (nums, target, answer) in test_case{
            let ans = Solution::two_sum(nums, target);
            assert_eq!(ans, answer);
        }
    }
}
