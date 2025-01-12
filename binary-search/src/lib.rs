pub fn find<T: Ord, V: AsRef<[T]>>(a: V, item: T) -> Option<usize> {
    let a = a.as_ref();

    let mut low = 0;
    let mut high = a.len();
    while low < high {
        let mid = (low + high) / 2;
        if a[mid] > item {
            high = mid;
        } else if a[mid] < item {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}
