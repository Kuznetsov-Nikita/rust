#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if arr.len() < k {
        vec![]
    } else if k == 0 {
        vec![vec![]]
    } else {
        let mut all_combinations = vec![];

        for i in 0..=arr.len() - k {
            let mut combinations = combinations(&arr[i + 1..], k - 1);

            for combination in &mut combinations {
                combination.insert(0, arr[i]);
            }

            all_combinations.append(&mut combinations);
        }

        all_combinations
    }
}
