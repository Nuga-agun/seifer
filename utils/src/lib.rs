/// Takes a list of bytes and swap byte number i with byte number j.
/// The indexes are modulo the list's size so that they are never out of range
/// # Example
/// ```
/// let mut list : [u8;5] = [1,2,3,4,5];
/// utils::swap_bytes(&mut list, 3, 1);
/// assert_eq!(list, [1,4,3,2,5]);
/// ```
pub fn swap_bytes(list: &mut [u8], i: usize, j: usize) {
    if list.len() == 0 {
        return;
    }
    let i = i%list.len();
    let j = j%list.len();
    let temp = list[i];
    list[i] = list[j];
    list[j] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_two() {
        let mut list: [u8;2] = [8,3];
        swap_bytes(&mut list, 0, 1);
        assert_eq!(list, [3,8]);
    }

    #[test]
    fn swap_with_index_out_of_range() {
        let mut list : [u8;5] = [1,2,3,4,5];
        swap_bytes(&mut list, 5, 12);
        assert_eq!(list, [3,2,1,4,5]);
    }

    #[test]
    fn swap_empty_list() {
        let mut list : [u8;0] = [];
        swap_bytes(&mut list, 1, 3);
        assert_eq!(list, []);
    }
}
