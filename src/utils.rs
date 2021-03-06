use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn system_time_to_timestamp(t: SystemTime) -> f64 {
    t.duration_since(UNIX_EPOCH).unwrap().as_micros() as f64 / 1_000_000_f64
}

pub fn timestamp_to_system_time(t: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(t)
}

pub fn current_system_time() -> SystemTime {
    SystemTime::now()
}

pub fn current_timestamp() -> f64 {
    system_time_to_timestamp(current_system_time())
}
