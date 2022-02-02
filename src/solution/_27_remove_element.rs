pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Solution::fast_slow_pointer(nums, val)
    }

    fn fast_slow_pointer(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1
            }
        }
        j as i32
    }


    fn collision_pointer(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            if nums[lo] == val {
                nums[lo] = nums[hi - 1];
                hi -= 1;
            } else {
                lo += 1
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
            (vec![3, 2, 2, 3], 3, 2),
            (vec![3, 2, 2, 3], 2, 2),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5),
            (vec![0], 1, 1),
            (vec![0], 0, 0),
        ];
        for (nums, val, answer) in test_case {
            let ans = Solution::remove_element(nums.clone().as_mut(), val);
            assert_eq!(answer, ans);
        }
    }
}
