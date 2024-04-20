mod bubble_sort;
pub use bubble_sort::bubble_sort;
mod insert_sort;
mod merge_sort;
mod select_sort;

pub use insert_sort::sort as insert_sort;
pub use select_sort::sort as select_sort;

pub use merge_sort::{merge, sort as merge_sort};
mod quick_sort;

pub use quick_sort::{partition, quick_sort_partition, sort as quick_sort};
