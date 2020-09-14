use std::time::SystemTime;
const CREATE_SECS: u64 = 1599939357;

pub fn secs_since_creation() -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // This hack will work for about 136 years
    (since_epoch - CREATE_SECS) as u32
}

