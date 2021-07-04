/// [Problem](https://leetcode-cn.com/problems/search-insert-position/)
/// ----------------------------------------------------------------------
///
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 你可以假设数组中无重复元素。
///
/// ```text
/// 示例 1:
///
/// 输入: [1,3,5,6], 5
/// 输出: 2
///
///
/// 示例 2:
///
/// 输入: [1,3,5,6], 2
/// 输出: 1
///
///
/// 示例 3:
///
/// 输入: [1,3,5,6], 7
/// 输出: 4
///
///
/// 示例 4:
///
/// 输入: [1,3,5,6], 0
/// 输出: 0
/// ```
///
/// Tips
/// -----
///
/// Answer
/// ------

pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < target {
                lo = mid + 1
            } else {
                hi = mid
            }
        }
        hi as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![1, 3, 4, 6], 0, 0, ),
            (vec![1, 3, 4, 6], 2, 1, ),
            (vec![1, 3, 4, 6], 5, 3, ),
            (vec![1, 3, 4, 6], 7, 4, ),
            (vec![4, 5, 5, 5], 5, 1, ),
            (vec![5, 5, 5, 5], 5, 0, ),
        ];


        for (nums, target, answer) in test_case {
            let ans = Solution::search_insert(nums, target);
            assert_eq!(ans, answer);
        }
    }
}
