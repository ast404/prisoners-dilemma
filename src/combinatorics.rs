pub struct Combinations {
    bins: u8,
    max_items_per_bin: u8,
    iterations_num: u64,
    current_iteration: u64,
}

impl Combinations {
    pub fn new(bins: u8, max_items_per_bin: u8) -> Self {
        Self {
            bins,
            max_items_per_bin,
            iterations_num: u64::from(max_items_per_bin).pow(u32::from(bins)),
            current_iteration: 0,
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration == self.iterations_num {
            return Option::None;
        }
        let mut current_iteration = self.current_iteration;
        self.current_iteration += 1;

        let mut bin_sizes: Self::Item = Vec::new();
        for _ in 0..self.bins {
            bin_sizes.push(
                (current_iteration % u64::from(self.max_items_per_bin))
                    .try_into()
                    .unwrap(),
            );
            current_iteration /= u64::from(self.max_items_per_bin);
        }
        Option::Some(bin_sizes)
    }
}
