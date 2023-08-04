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


pub fn manipulate_data(seed: u64, data: Vec<u8>) -> Vec<u8> {
    // manipulate_data_timebased(seed, data)
    manipulate_data_byteshuffle(seed, data)
}

pub fn manipulate_data_byteshuffle(seed: u64, mut data: Vec<u8>) -> Vec<u8> {
    let inverse = !seed;
    let shifted = inverse.rotate_left(2); // i have no clue why 2, just initial start point
    let mut seed_perm = seed ^ shifted; // xor is always done by the cool kids. let's see where we get to
    let tot = data.len();
    for i in 0usize..tot {
        let rotate_amount: u32 = 1.max(i as u32);
        // there might be some clash of when rotating left and right that we get 
        // identical permutations at some point, but we'll figure it out later.
        let mut value = data[i] as u64;
        value ^= seed_perm;
        data[i] = (value & 0xFF) as u8;
        // data[i] = data[i] ^ seed_perm;
        // let combowombo = (data[i] as u64) + in_seconds * seed - sub;
        // extract 8 x u8 from one u64
        seed_perm = seed.rotate_right(rotate_amount) ^ seed_perm.rotate_left(rotate_amount);
    }
    data
}

pub fn manipulate_data_timebased(seed: u64, mut data: Vec<u8>) -> Vec<u8> {
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
        // extract 8 x u8 from one u64
        let bytes = u64_to_u8_array(combowombo);
        for j in 0..8 {
            data[i+j] = prev[(j+(seed as usize)) % 8] ^ bytes[(j+(seed as usize)) % 8];
        }
        for j in 0..8 {
            prev[j] = data[i+j];
        }
    }
    data
}

// //chatgpt
// fn bytes_to_u64(bytes: &[u8]) -> Option<u64> {
//     if bytes.len() < 8 {
//         return None;
//     }

//     let mut value: u64 = 0;
//     for &byte in &bytes[0..8] {
//         value = (value << 8) | byte as u64;
//     }

//     Some(value)
// }


fn u64_to_u8_array(combowombo: u64) -> [u8; 8] {
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
    bytes
}

