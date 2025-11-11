pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // search range
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = (low + high) / 2;
        let mid_value = array[mid];

        if mid_value == key {
            return Some(mid); 
        } else if mid_value < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    // Not found
    None
}


