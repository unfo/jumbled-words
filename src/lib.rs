use std::vec::Vec;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn manipulate_data(seed: u64, mut data: Vec<u8>) -> Vec<u8> {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_seconds = since_the_epoch.as_secs();
    let subsec_nanos = since_the_epoch.subsec_nanos() as u64;
    for i in (0..(data.len())).step_by(8) {
        let combowombo = ((data[i] as u64) + in_seconds * seed) % subsec_nanos;
        let bytes: [u8; 8] = [
            ((combowombo >> 56) & 0xFF) as u8,
            ((combowombo >> 48) & 0xFF) as u8,
            ((combowombo >> 40) & 0xFF) as u8,
            ((combowombo >> 32) & 0xFF) as u8,
            ((combowombo >> 24) & 0xFF) as u8,
            ((combowombo >> 16) & 0xFF) as u8,
            ((combowombo >> 8)  & 0xFF) as u8,
            ((combowombo     )  & 0xFF) as u8,
        ];
        for j in 0..8 {
            data[i+j] = bytes[j];
        }
    }
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
