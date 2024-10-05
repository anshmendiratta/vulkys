pub fn merge_sort<T: Ord + Clone + Copy>(array: &mut [T]) {
    let n = array.len();
    if n < 2 {
        return;
    }

    let mid = n / 2;

    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    let mut merged = Vec::with_capacity(n);
    let (mut i, mut j) = (0, mid);

    while i < mid && j < n {
        if array[i] <= array[j] {
            merged.push(array[i]);
            i += 1;
        } else {
            merged.push(array[j]);
            j += 1;
        }
    }

    while i < mid {
        merged.push(array[i]);
        i += 1;
    }
    while j < n {
        merged.push(array[j]);
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
