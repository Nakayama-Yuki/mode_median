// 整数のリストが与えられ、ベクターを使ってmdedian(中央値)とmode(最頻値)を返す関数

const NUMBERS: &[i32] = &[1, 5, 4, 9, 8, 7, 2, 3, 6, 1, 7];

fn main() {
    let median = calculate_median(NUMBERS);
    let mode = calculate_mode(NUMBERS);
    println!("Median: {}", median);
    println!("Mode: {:?}", mode);
}

// 中央値: 値を小さい順に並べ、要素数が奇数なら真ん中の値、
// 偶数なら中央2つの平均を返す。
fn calculate_median(numbers: &[i32]) -> f64 {
    // .to_vec()でnumbersをベクターに変換して、sorted_numbersに格納する
    let mut sorted_numbers = numbers.to_vec();
    // .sort()でsorted_numbersを昇順に並べ替える
    sorted_numbers.sort();
    // .len()でsorted_numbersの要素数を取得して、lenに格納する
    let len = sorted_numbers.len();
    // 要素数が偶数なら、中央2つの平均を計算して返す。
    if len % 2 == 0 {
        (sorted_numbers[len / 2 - 1] as f64 + sorted_numbers[len / 2] as f64) / 2.0
    // 奇数なら、真ん中の値を返す
    } else {
        sorted_numbers[len / 2] as f64
    }
}

// 最頻値: 各値の出現回数を数え、最も多く出た値をすべて返す。
// 同率1位が複数ある場合は、その値をまとめて Vec に入れる。
fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
    // 新規ハッシュマップを生成して、frequencyに格納する
    let mut frequency = std::collections::HashMap::new();
    // numbersの各要素について、frequencyに出現回数をカウントし、frequencyに格納する
    // .entry()でnumberをキーとしてfrequencyにアクセスし、.or_insert(0)で初期値を0に設定する。
    //  numbers = [1, 2, 2, 3] なら、frequency = {1: 1, 2: 2, 3: 1}
    for &number in numbers {
        *frequency.entry(number).or_insert(0) += 1;
    }
    // frequencyの値の中で最大の出現回数を取得して、max_frequencyに格納する
    let max_frequency = frequency.values().cloned().max().unwrap_or(0);
    // 最頻値を計算してVecに格納する
    frequency
        .into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .map(|(number, _)| number)
        .collect()
}
