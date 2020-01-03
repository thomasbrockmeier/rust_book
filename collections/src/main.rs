fn main() {
    descriptives::main();
}

pub mod descriptives {
    use std::collections::HashMap;

    pub fn main() {
        let mut vec: Vec<usize> = vec![3, 4, 2, 6, 3, 1, 3, 5, 7, 1, 4, 3, 6, 4, 3];
        vec.sort();

        println!("Mean:     {}", mean(&vec));
        println!("Median:   {}", median(&vec));
        println!("Mode:     {}", mode(&vec));
    }

    fn mean(vec: &Vec<usize>) -> usize {
        let mut acc: usize = 0;
        for i in vec {
            acc += i;
        }
        return acc / vec.len()
    }

    fn median(vec: &Vec<usize>) -> usize {
        return vec[vec.len() / 2]
    }

    fn frequencies(vec: &Vec<usize>) -> std::collections::HashMap<&usize, usize> {
        let mut frequencies = HashMap::new();
        for i in vec {
            let count = frequencies.entry(i).or_insert(0);
            *count += 1;
        }
        return frequencies;
    }

    fn mode(vec: &Vec<usize>) -> usize {
        // Ignores cases where there are multiple modes
        let mut max_key = 0;
        let mut max_val = 0;
        for (k, v) in frequencies(&vec) {
            if v > max_val {
                max_key = *k;
                max_val = v;
            }
        }
        return max_key;
    }
}

