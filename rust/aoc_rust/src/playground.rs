use std::collections::HashMap;

fn median(mut nums: Vec<i32>) -> Option<i32> {
    let len = nums.len();
    if len < 2 {
        return None;
    }
    nums.sort_unstable();
    return if len % 2 == 0 {
        // even
        Some((nums[len / 2] + nums[len / 2 - 1]) / 2)
    } else {
        // odd
        Some(nums[len / 2])
    };
}

fn mode(nums: Vec<i32>) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }
    let mut occurences = HashMap::new();
    for x in nums {
        let entry = occurences.entry(x).or_insert(0);
        *entry += 1;
    }
    Some(
        *occurences.iter()
        .max_by_key(|item| *item.1)
        .map(|item| item.0).unwrap()
    )
}

fn mode_better(nums: Vec<i32>) -> Option<i32> {
    let mut counts = HashMap::new();

    nums.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}
