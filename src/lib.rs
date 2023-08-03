use std::vec::Vec;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_random_data(megabytes: usize, seed: u64) -> Vec<u8> {
    // Create a data structure with the specified number of bytes
    let mut data = vec![0u8; megabytes * 1024 * 1024];

    // Apply a function to manipulate the data (identity function in this case)
    let rounds = 1u64;
    for round in 0..rounds {
        data = manipulate_data(seed + round, data);
    }
    data
}

pub fn manipulate_data(seed: u64, mut data: Vec<u8>) -> Vec<u8> {
    let tot = data.len();
    let mut prev:[u8; 8] = [0,0,0,0, 0,0,0,0];

    for i in (0..tot).step_by(8) {
        let now = SystemTime::now();
        let since_the_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let in_seconds = since_the_epoch.as_secs();
        let subsec_nanos = since_the_epoch.subsec_nanos();
        let sub = ((subsec_nanos as u64) << 32) | (subsec_nanos as u64);
        let combowombo = (data[i] as u64) + in_seconds * seed - sub;
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
            data[i+j] = prev[(j+(seed as usize)) % 8] ^ bytes[(j+(seed as usize)) % 8];
        }
        for j in 0..8 {
            prev[j] = data[i+j];
        }
    }
    data
}

