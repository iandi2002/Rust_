pub fn sort(array: &mut [i32]) {
    let len = array.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}
