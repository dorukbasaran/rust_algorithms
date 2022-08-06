// Worst-case performance: O(log n)
// Best-case performance: O(1) when the central index would directly match the desired value.
// Average performance: O(log n)

pub fn binary_search<T: PartialEq + PartialOrd>(input: &Vec<T>, desired_value: T) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = input.len() - 1;
    let mut mid;

    loop {
        mid = (upper_bound + lower_bound) / 2;
        if lower_bound > upper_bound {
            return None;
        } else if input[mid] == desired_value {
            return Some(mid);
        } else if input[mid] < desired_value {
            lower_bound = mid + 1;
        } else if input[mid] > desired_value {
            upper_bound = mid - 1;
        }
    }
}

#[cfg(test)]
mod binary_search_tests {
    use crate::search::binary_search;

    #[test]
    fn binary_search_int() {
        let mut random_vector: Vec<i32> = Vec::new();
        for number in 0..10 {
            random_vector.push(number);
        }
        let index = binary_search::binary_search(&random_vector, 4).unwrap();
        assert_eq!(index, 4);
    }

    #[test]
    fn binary_search_str() {
        let mut random_vector: Vec<&str> = Vec::new();
        random_vector.push("A");
        random_vector.push("D");
        random_vector.push("K");
        random_vector.push("O");
        let index = binary_search::binary_search(&random_vector, "D").unwrap();
        assert_eq!(index, 1);
    }

    #[test]
    #[should_panic]
    fn binary_search_not_in_vec() {
        let mut random_vector: Vec<i32> = Vec::new();
        for number in 0..10 {
            random_vector.push(number);
        }
        // 12 is not in input vec so function return None and must be panic at unwrap
        let index = binary_search::binary_search(&random_vector, 12).unwrap();
        assert_eq!(index, 4);
    }
}
