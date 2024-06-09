use std::iter::successors;

fn fill_bins(iteration: u64, bins: u8, max_items_per_bin: u8) -> Vec<u8> {
    successors(Some(iteration), |current| {
        Some(current / u64::from(max_items_per_bin))
    })
    .take(bins.into())
    .map(|current| -> u8 { (current % u64::from(max_items_per_bin)).try_into().unwrap() })
    .collect()
}

pub fn get_combinations(bins: u8, max_items_per_bin: u8) -> impl Iterator<Item = Vec<u8>> {
    let iterations_num = u64::from(max_items_per_bin).pow(u32::from(bins));
    (0..iterations_num).map(move |iteration| fill_bins(iteration, bins, max_items_per_bin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterations_number() {
        assert_eq!(get_combinations(4, 5).count(), 625);
        assert_eq!(get_combinations(5, 5).count(), 3125);
        assert_eq!(get_combinations(6, 5).count(), 15625);
        assert_eq!(get_combinations(3, 6).count(), 216);
        assert_eq!(get_combinations(4, 6).count(), 1296);
        assert_eq!(get_combinations(5, 6).count(), 7776);
    }

    #[test]
    fn all_binary_combinations() {
        let combinations: Vec<_> = get_combinations(2, 2).collect();
        assert_eq!(combinations.len(), 4);
        assert!(combinations.contains(&vec!(0u8, 0u8)));
        assert!(combinations.contains(&vec!(1u8, 0u8)));
        assert!(combinations.contains(&vec!(0u8, 1u8)));
        assert!(combinations.contains(&vec!(1u8, 1u8)));
    }

    #[test]
    fn single_combination() {
        let combinations: Vec<_> = get_combinations(4, 1).collect();
        assert_eq!(combinations.len(), 1);
        assert!(combinations.contains(&vec!(0u8, 0u8, 0u8, 0u8)));
    }

    #[test]
    fn some_combinations() {
        let combinations: Vec<_> = get_combinations(4, 6).collect();
        assert!(combinations.contains(&vec!(0u8, 0u8, 0u8, 0u8)));
        assert!(combinations.contains(&vec!(5u8, 0u8, 0u8, 0u8)));
        assert!(combinations.contains(&vec!(1u8, 1u8, 1u8, 1u8)));
        assert!(combinations.contains(&vec!(5u8, 5u8, 5u8, 5u8)));
        assert!(combinations.contains(&vec!(2u8, 3u8, 1u8, 4u8)));

        assert_eq!(false, combinations.contains(&vec!(0u8, 0u8, 0u8, 6u8)));
        assert_eq!(false, combinations.contains(&vec!(6u8, 6u8, 6u8, 6u8)));
    }
}
