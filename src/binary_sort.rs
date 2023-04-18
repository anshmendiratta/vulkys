fn binary_search<T: std::cmp::PartialOrd>(L: Vec<T>, item: T) -> usize {
    let value: &T = &L[0];
    let mut front_idx: usize = 0;
    let mut back_idx: usize = L.len();
    let mid: usize = 0;
    
    while value != &item {
        let mid: usize = (front_idx + back_idx) / 2;
        
        if L[mid] > item {
            front_idx = mid;
        } else if L[mid] < item {
            back_idx = mid;
        } else {
            return mid
        }
    }   
    mid
}

// function binary_search(A, n, T) is
//     L := 0
//     R := n − 1
//     while L ≤ R do
//         m := floor((L + R) / 2)
//         if A[m] < T then
//             L := m + 1
//         else if A[m] > T then
//             R := m − 1
//         else:
//             return m
//     return unsuccessful