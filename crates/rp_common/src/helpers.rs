pub fn equal_vecs<T>(first: &Vec<T>, second: &Vec<T>) -> bool
where
    T: Ord + Clone,
{
    if first.len() != second.len() {
        return false;
    }

    let mut first = first.clone();
    let mut second = second.clone();

    first.sort();
    second.sort();

    first == second
}
