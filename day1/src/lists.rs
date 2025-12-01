pub struct Lists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

impl Lists {
    pub fn get_lists(&mut self, input: Vec<String>) {
        for couple in &input {
            let numbers: Vec<i32> = couple
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            // println!("{:?}", numbers);
            self.left.push(numbers[0]);
            self.right.push(numbers[1]);
        }
    }
    // pub fn print_lists(&self) {
    //     let mut count = 0;
    //     loop {
    //         println!("{:?} | {:?}", self.left[count], self.right[count]);
    //         count += 1;
    //         if count == self.left.len() - 1 {
    //             break;
    //         }
    //     }
    // }
    pub fn sort_lists(&mut self) {
        self.left.sort();
        self.right.sort();
    }
    pub fn get_dist_sum(&mut self) -> i32 {
        let mut sum = 0;
        loop {
            sum += (self.right.remove(0) - self.left.remove(0)).abs();
            if self.left.is_empty() {
                break;
            }
        }
        sum
    }
    pub fn get_similarity(&self) -> i32 {
        let mut sym = 0;

        for &n in &self.left {
            match self.right.binary_search(&n) {
                Ok(mut index) => {
                    while index > 0 && self.right[index - 1] == n {
                        index -= 1;
                    }
                    let mut count = 0;
                    loop {
                        if self.right[index] != n || index == self.right.len() - 1 {
                            break;
                        }
                        index += 1;
                        count += 1;
                    }
                    sym += n * count;
                },
                Err(_) => { continue; },
            };
        }
        sym
    }
}
