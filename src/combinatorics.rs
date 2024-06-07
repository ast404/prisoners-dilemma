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
