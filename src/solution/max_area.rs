use super::*;
use std::cmp;

impl Solution {
    /// # [11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `i^th` line are `(i, 0)` and `(i, height[i])`.
    ///
    /// Find two lines that together with the x-axis form a container, such that the container contains the most water.
    ///
    /// Return the maximum amount of water a container can store.
    ///
    /// **Notice**  that you may not slant the container.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;">
    ///
    /// ```
    /// Input: height = [1,8,6,2,5,4,8,3,7]
    /// Output: 49
    /// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: height = [1,1]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == height.length`
    /// - `2 <= n <= 10^5`
    /// - `0 <= height[i] <= 10^4`
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_area = 0;
        while i < j {
            let left_height = height[i];
            let right_height = height[j];
            if left_height < right_height {
                let new_area = (j - i) as i32 * left_height;
                max_area = cmp::max(max_area, new_area);
                i = (i + 1..j).find(|k| height[*k] > left_height).unwrap_or(j);
            } else {
                let new_area = (j - i) as i32 * right_height;
                max_area = cmp::max(max_area, new_area);
                j = (i + 1..j)
                    .rev()
                    .find(|k| height[*k] > right_height)
                    .unwrap_or(i);
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
