pub fn equal_vecs<T>(first: &[T], second: &[T]) -> bool
where
    T: Ord + Clone,
{
    if first.len() != second.len() {
        return false;
    }

    let mut first = first.to_owned();
    let mut second = second.to_owned();

    first.sort();
    second.sort();

    first == second
}
