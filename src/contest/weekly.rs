use std::collections::HashMap;

use leetcode_practices::show_time_cost;

pub(crate) fn contest() {
    println!("================================================ Contest Weekly Start! ================================================");
    weekly_contest_439();
    println!("================================================ Contest Weekly   End! ================================================");
}

fn weekly_contest_439() {
    question_3471();
}

fn question_3471() {
    struct Solution;

    impl Solution {
        pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
            if let Ok(loc) = k.try_into() {
                let mut map = HashMap::new();
                for core in nums.windows(loc) {
                    core.iter().for_each(|&e| {
                        *map.entry(e).or_insert(0) += 1;
                    });
                }
                map.iter()
                    .filter(|(_, &v)| v == 1 || loc == nums.len())
                    .map(|(&k, _)| k)
                    .max()
                    .unwrap_or(-1)
            } else {
                panic!("ERROR: Can not cast argument k: i32 = {} to usize!", k);
            }
        }
    }

    assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
    assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 1), 9);
    assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
    assert_eq!(Solution::largest_integer(vec![0, 0], 2), 0);

    show_time_cost!(Solution::largest_integer, vec![3, 9, 2, 1, 7], 3);
    show_time_cost!(Solution::largest_integer, vec![3, 9, 7, 2, 1, 7], 4);
    show_time_cost!(Solution::largest_integer, vec![3, 9, 7, 2, 1, 7], 1);
    show_time_cost!(Solution::largest_integer, vec![0, 0], 1);
    show_time_cost!(Solution::largest_integer, vec![0, 0], 2);

    struct Solution1;

    impl Solution1 {
        pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
            if let Ok(loc) = k.try_into() {
                nums.windows(loc)
                    .flat_map(|core| core.iter())
                    .fold(HashMap::new(), |mut map, &e| {
                        *map.entry(e).or_insert(0) += 1;
                        map
                    })
                    .iter()
                    .filter(|(_, &v)| v == 1 || loc == nums.len())
                    .map(|(&k, _)| k)
                    .max()
                    .unwrap_or(-1)
            } else {
                panic!("ERROR: Can not cast argument k: i32 = {} to usize!", k);
            }
        }
    }

    assert_eq!(Solution1::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    assert_eq!(Solution1::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
    assert_eq!(Solution1::largest_integer(vec![3, 9, 7, 2, 1, 7], 1), 9);
    assert_eq!(Solution1::largest_integer(vec![0, 0], 1), -1);
    assert_eq!(Solution1::largest_integer(vec![0, 0], 2), 0);

    show_time_cost!(Solution1::largest_integer, vec![3, 9, 2, 1, 7], 3);
    show_time_cost!(Solution1::largest_integer, vec![3, 9, 7, 2, 1, 7], 4);
    show_time_cost!(Solution1::largest_integer, vec![3, 9, 7, 2, 1, 7], 1);
    show_time_cost!(Solution1::largest_integer, vec![0, 0], 1);
    show_time_cost!(Solution1::largest_integer, vec![0, 0], 2);
}
