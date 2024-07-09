/// A recursive implementation of merge sort
pub fn merge_sort<T: Ord + Clone + Copy>(array: &mut [T]) {
    let n = array.len();
    if n < 2 {
        return;
    }

    let mid = n / 2;

    // Keep recursively dividing until the length of the array is less than two
    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    let mut merged = Vec::with_capacity(n);
    let (mut i, mut j) = (0, mid);

    // Checking that the whole of the two arrays haven't been iterated through (short-circuits when one of the arrays has finished)
    while i < mid && j < n {
        // Comparing the first elements of each array and pushing the smaller one
        // The use of clone so that the ownership of items in `array` doesn't change
        if array[i] <= array[j] {
            merged.push(array[i].clone());
            i += 1;
        } else {
            merged.push(array[j].clone());
            j += 1;
        }
    }
    // Otherwise if one of the arrays being used to compare elements has finished, finish the other one
    while i < mid {
        merged.push(array[i].clone());
        i += 1;
    }
    // Otherwise if one of the arrays being used to compare elements has finished, finish the other one
    while j < n {
        merged.push(array[j].clone());
        j += 1;
    }
    array.copy_from_slice(&merged);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_merge_sort() {
        let mut a = [1, 2, 3, 4, 5, 6, 243, 23, 4];
        merge_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 4, 5, 6, 23, 243])
    }
}
