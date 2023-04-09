// time complexity is O(sqrt(n))
pub fn sqrt_root<T: PartialEq + PartialOrd>(input: &Vec<T>, desired_value: T) -> Option<usize> {
    let jump_amount = (input.len() as f64).sqrt() as usize;
    if jump_amount == 0 {
        return None;
    }
    let mut i = 0;
    while i < input.len() {
        if input[i] >= desired_value {
            break;
        }
        let new_i = i + jump_amount;
        if new_i >= input.len() {
            i = input.len() - 1;
            break;
        }
        i = new_i;
    }
    let j: usize;
    if jump_amount > i {
        j = 0;
    } else {
        j = i - jump_amount;
    }

    for j in j..=i {
        if input[j] == desired_value {
            return Some(j);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_root() {
        // Test case 1: Empty vector
        let input1: Vec<i32> = vec![];
        assert_eq!(sqrt_root(&input1, 1), None);

        // Test case 2: Vector with only one element
        let input2 = vec![2];
        assert_eq!(sqrt_root(&input2, 2), Some(0));
        assert_eq!(sqrt_root(&input2, 1), None);

        // Test case 3: Vector with multiple elements
        let input3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sqrt_root(&input3, 5), Some(4));
        assert_eq!(sqrt_root(&input3, 11), None);

        // Test case 4: Vector with duplicates
        let input4 = vec![1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sqrt_root(&input4, 4), Some(3));
        assert_eq!(sqrt_root(&input4, 11), None);
    }
}
