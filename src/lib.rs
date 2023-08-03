pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn manipulate_data(data: Vec<u8>) -> Vec<u8> {
    // Future iterations can modify this function to apply different manipulations
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
