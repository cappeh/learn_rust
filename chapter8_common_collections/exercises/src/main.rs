use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 7, 98, 7, 73, 98, 4, 1, 7, 52, 18, 32, 4, 76];
    numbers.sort();

    let median = if numbers.len() % 2 == 1 {
        numbers[numbers.len() / 2]
    } else {
        let mid = numbers.len() / 2;
        (numbers[mid - 1] + numbers[mid]) / 2
    };

    println!("Median: {median}");
    
    let mut number_map = HashMap::new();

    for &num in &numbers {
        let count = number_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max_value = numbers[0];
    let mut max_count = 0;

    for (&key, &val) in &number_map {
        if val > max_count {
            max_count = val;
            max_value = key;
        }
    }

    println!("Mode: {max_value}: {max_count}");

    println!("{number_map:?}");
}
