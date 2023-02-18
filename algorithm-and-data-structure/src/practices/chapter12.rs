use superslice::Ext;

pub fn rank(v: Vec<u32>, i: usize) -> usize {
    let mut vec = v.to_vec();
    let search = vec[i - 1];
    vec.sort();
    vec.upper_bound(&search)
}

struct Store {
    stock: u32,
    price: u32,
}

pub fn energy_drink_collector(stocks: Vec<u32>, prices: Vec<u32>, mut amount: u32) -> u32 {
    let mut stores = vec![];
    for i in 0..stocks.len() {
        stores.push(Store {
            stock: stocks[i],
            price: prices[i],
        });
    }
    let mut total_price = 0;
    stores.sort_by_key(|v| v.price);
    for store in stores {
        let c = amount.min(store.stock);
        amount -= c;
        total_price += store.price * c;
    }
    total_price
}

#[cfg(test)]
mod test {
    use super::{energy_drink_collector, rank};

    #[test]
    fn test_rank() {
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 2), 5);
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 1), 1);
        assert_eq!(rank(vec![1, 5, 4, 3, 2], 3), 4);
    }

    #[test]
    fn test_energy_drink_collector() {
        assert_eq!(energy_drink_collector(vec![9, 4], vec![4, 2], 5), 12);
    }
}
