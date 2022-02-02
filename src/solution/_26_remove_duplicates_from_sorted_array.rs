pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 0{
            return nums.len() as i32;
        }

        let mut j = 0;
        for i in 0..nums.len(){
            if nums[j] < nums[i]{
                nums[j+1] = nums[i];
                j += 1
            }
        }
        (j+1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_case = vec![
            (vec![2, 2, 3, 3], 2),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 5),
            (vec![0], 1),
            (vec![], 0),
        ];
        for (nums, answer) in test_case {
            let ans = Solution::remove_duplicates(nums.clone().as_mut());
            assert_eq!(answer, ans);
        }
    }
}
