// Worst-case performance: O(n)
// Best-case performance: O(1)
// Average performance: O(log(log(n)))[2]

// Interpolation search is an algorithm for searching for a key in an array
// that has been ordered by numerical values assigned to the keys (key values)
// !! so interpolations only be applied on numeric types !!

pub fn interpolation_search(input: &Vec<usize>, desired_value: usize) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = input.len() - 1;
    let mut mid_point: usize;

    loop {
        if lower_bound == upper_bound || input[lower_bound] == input[upper_bound] {
            return None;
        }
        mid_point = lower_bound
            + ((upper_bound - lower_bound)
                / (input[upper_bound] as usize - input[lower_bound] as usize))
                * (desired_value - input[lower_bound]) as usize;

        if input[mid_point] == desired_value {
            return Some(mid_point);
        } else {
            if input[mid_point] < desired_value {
                lower_bound = mid_point + 1;
            } else if input[mid_point] > desired_value {
                upper_bound = mid_point - 1;
            }
        }
    }
}

#[cfg(test)]
mod interpolation_search_tests {
    use crate::search::interpolation_search;

    #[test]
    fn search() {
        let mut random_vector: Vec<usize> = Vec::new();
        for number in 0..10 {
            random_vector.push(number);
        }
        let index = interpolation_search::interpolation_search(&random_vector, 4).unwrap();
        assert_eq!(index, 4);
    }

    #[test]
    #[should_panic]
    fn search_panic() {
        let mut random_vector: Vec<usize> = Vec::new();
        for number in 0..10 {
            random_vector.push(number);
        }
        let index = interpolation_search::interpolation_search(&random_vector, 12).unwrap();
        assert_eq!(index, 4);
    }
}
