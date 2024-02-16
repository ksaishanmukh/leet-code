use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut table: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        table.insert(nums[i], i as i32);
    }

    for i in 0..nums.len() {
        let rem = target - nums[i];
        if table.contains_key(&rem) {
            let j = *table.get(&rem).unwrap();
            if j != 1 {
                return vec![i as i32, j];
            }
        }
    }

    vec![]
}
