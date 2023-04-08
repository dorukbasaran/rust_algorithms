// Time complexity O(n^2)
// Space complexty O(1)

pub fn bubble_sort<T: PartialOrd>(input: &mut Vec<T>) {
    let mut swapped_something: bool;
    for i in 0..input.len() {
        swapped_something = false;
        for j in 0..input.len() - 1 - i {
            if input[j] > input[j + 1] {
                swapped_something = true;
                input.swap(j, j + 1);
            }
        }
        if !swapped_something {
            break;
        }
    }
}

#[cfg(test)]
mod bubble_sort_test {
    use crate::sort::bubble_sort;

    #[test]
    fn sort() {
        let mut input = vec![-3, 3, 2, 65, 23, 41, 22];
        bubble_sort::bubble_sort(&mut input);
        assert_eq!(input, vec![-3, 2, 3, 22, 23, 41, 65]);
    }

    #[test]
    fn sort_string() {
        let mut input = vec!["Mithrandir", "Elessar", "Gimli", "Elrond"];
        bubble_sort::bubble_sort(&mut input);
        assert_eq!(input, vec!["Elessar", "Elrond", "Gimli", "Mithrandir"])
    }
}
