pub fn readfile(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = readfile(2, 2);
        assert_eq!(result, 4);
    }
}
