// 整数のリストが与えられ、ベクターを使ってmdedian(中央値)とmode(最頻値)を返す関数

const NUMBERS: &[i32] = &[1, 2, 3, 4, 5, 5];

fn main() {
    let median = calculate_median(NUMBERS);
    let mode = calculate_mode(NUMBERS);
    println!("Median: {}", median);
    println!("Mode: {:?}", mode);
}

// 中央値を計算する関数
fn calculate_median(numbers: &[i32]) -> f64 {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    let len = sorted_numbers.len();
    if len % 2 == 0 {
        (sorted_numbers[len / 2 - 1] as f64 + sorted_numbers[len / 2] as f64) / 2.0
    } else {
        sorted_numbers[len / 2] as f64
    }
}

// 最頻値を計算する関数
fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
    let mut frequency = std::collections::HashMap::new();
    for &number in numbers {
        *frequency.entry(number).or_insert(0) += 1;
    }
    let max_frequency = frequency.values().cloned().max().unwrap_or(0);
    frequency
        .into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .map(|(number, _)| number)
        .collect()
}
