pub fn swap_bytes(list: &mut [u8], i: usize, j: usize) {
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
}
