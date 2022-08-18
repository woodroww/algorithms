
fn max_profit(prices: Vec<i32>) -> i32 {
    println!("prices: {:?}", prices);
    let mut buy = i32::MAX;
    let mut sell = 0;
    for i in 0..prices.len() {
        println!("buy: min({}, {}), sell: max({}, {})", buy, prices[i], sell, prices[i] - buy);
        buy = std::cmp::min(buy, prices[i]);
        sell = std::cmp::max(sell, prices[i] - buy);
        println!("buy: {}, sell: {}", buy, sell);
    }
    println!();
    return sell;
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("Max profit: {}", max_profit(prices));
}

#[test]
fn initial_test() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);
}

#[test]
fn another_test() {
    let prices = vec![2, 4, 1];
    assert_eq!(max_profit(prices), 2);
}

/*#[test]
fn bit_test() {

    use std::fs;

    let contents = fs::read_to_string("data.txt")
        .expect("Couldn't read file");
    let nums: Vec<i32> = contents
        .split(',')
        .map(|s| std::str::FromStr::from_str(s).unwrap())
        .collect();
}*/
