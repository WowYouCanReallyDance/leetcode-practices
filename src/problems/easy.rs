use std::collections::HashMap;

use leetcode_practices::show_time_cost;

pub(crate) fn questions() {
    println!("================================================ Problems Easy Start! ================================================");
    easy_2275();
    easy_1();
    println!("================================================ Problems Easy   End! ================================================");
}

fn easy_2275() {
    fn largest_combination(candidates: Vec<i32>) -> i32 {
        (0..24)
            .map(|e| candidates.iter().filter(|&&e1| e1 & (1 << e) != 0).count() as i32)
            .max()
            .unwrap()
    }

    let vec = vec![
        33, 93, 31, 99, 74, 37, 3, 4, 2, 94, 77, 10, 75, 54, 24, 95, 65, 100, 41, 82, 35, 65, 38,
        49, 85, 72, 67, 21, 20, 31,
    ];

    let largest = show_time_cost!(largest_combination, vec);
    println!("Largest length is: {}", largest);
}

fn easy_1() {
    struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut result = Vec::new();
            let mut hash = HashMap::<i32, usize>::new();
            nums.iter().enumerate().for_each(|(i, &e)| {
                let k = target - e;
                if let Some(v) = hash.get(&k) {
                    result.push(*v as i32);
                    result.push(i as i32);
                }
                hash.insert(e, i);
            });
            result
        }
    }

    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("Result is: {:?}", result);
}
