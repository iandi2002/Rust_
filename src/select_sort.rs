pub fn sort(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i+1..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}
