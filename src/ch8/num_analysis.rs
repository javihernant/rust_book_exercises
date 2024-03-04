use std::collections::HashMap;

fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.to_owned();
    v.sort();
    let mid = v.len().div_ceil(2);
    v[mid - 1]
}

fn mode(vs: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for v in vs {
        let count = map.entry(v).or_insert(0);
        *count += 1;
    }
    let max = map.iter()
        .max_by_key(|entry| entry.1)
        .map(|entry| entry.0)
        .unwrap();
    **max
}

pub fn median_and_mode(v: Vec<i32>) {
    let med = median(&v);
    println!("Median: {}", med);
    let mode = mode(&v);
    println!("Mode: {}", mode);
}

#[cfg(test)]
mod tests {
    use crate::ch8::num_analysis::median;
    #[test]
    fn median_works() {
        assert_eq!(median(vec![5, 1, 42, 4, 25].as_ref()), 5);
        assert_eq!(median(vec![5, 1, 42, 4].as_ref()), 4);
    }
    use crate::ch8::num_analysis::mode;
    #[test]
    fn mode_works() {
        assert_eq!(mode(vec![5, 5, 5, 4, 3, 3].as_ref()), 5);
    }
}
