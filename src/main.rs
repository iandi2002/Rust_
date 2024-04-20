use sorting_functions::bubble_sort;
use sorting_functions::insert_sort;
use sorting_functions::merge_sort;
use sorting_functions::quick_sort;
use sorting_functions::select_sort;
fn main() {
    let mut numbers = vec![400, 20, 10, 30];
    bubble_sort(&mut numbers);
    println!("bubble sort: {:?}", numbers);

    let mut numbers = vec![400, 20, 10, 30];
    merge_sort(&mut numbers);
    println!("merge sort: {:?}", numbers);

    let mut numbers = vec![400, 20, 10, 30];
    quick_sort(&mut numbers);
    println!("quick sort: {:?}", numbers);

    let mut numbers = vec![400, 20, 10, 30];
    select_sort(&mut numbers);
    println!("select sort: {:?}", numbers);

    let mut numbers = vec![400, 20, 10, 30];
    insert_sort(&mut numbers);
    println!("insert sort: {:?}", numbers);
}
