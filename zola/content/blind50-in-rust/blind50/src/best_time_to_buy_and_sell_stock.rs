// O(n)

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut current_profit = 0;
    for i in 0..prices.len() - 1 {
        let price = prices.get(i).unwrap();
        let next_price = prices.get(i + 1).unwrap();
        let incremental_profit = next_price - price;
        current_profit += incremental_profit;
        if current_profit < 0 {
            current_profit = 0;
        }
        if current_profit > max_profit {
            max_profit = current_profit;
        }
    }
    max_profit
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn test_2() {
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
