use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct ProgressCalculator {
    /// total size in bytes.
    total_size_in_bytes: u64,

    /// loaded size in bytes.
    loaded_size_in_bytes: u64,

    /// timestamp of start time.
    start_time: Option<u64>,

    /// timestamp of last loaded time.
    last_loaded_time: Option<u64>,

    /// instantaneous speed of loading
    instantaneous_speed: f64,
}

impl ProgressCalculator {
    pub fn new(total_size_in_bytes: u64) -> Self {
        ProgressCalculator {
            total_size_in_bytes,
            loaded_size_in_bytes: 0,
            start_time: None,
            last_loaded_time: None,
            instantaneous_speed: 0.0,
        }
    }

    fn calculate_instantaneous_speed(&self, size_in_bytes: u64) -> f64 {
        let now = get_now_timestamp();
        if let Some(last_loaded_time) = self.last_loaded_time {
            let duration_seconds = now - last_loaded_time;
            (size_in_bytes as f64 / duration_seconds as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn is_finished(&self) -> bool {
        self.loaded_size_in_bytes == self.total_size_in_bytes
    }

    pub fn get_progress(&self) -> String {
        let raw_progress =
            (self.loaded_size_in_bytes as f64 / self.total_size_in_bytes as f64) * 100.0;
        let formatted_progress = format!("{:.2}%", raw_progress);

        formatted_progress.to_string()
    }

    pub fn get_instantaneous_speed(&self) -> String {
        format_speed(self.instantaneous_speed)
    }

    pub fn get_average_speed(&self) -> String {
        if let (Some(start_time), Some(last_loaded_time)) = (self.start_time, self.last_loaded_time)
        {
            let duration_seconds = last_loaded_time - start_time;
            let raw_average_speed =
                (self.loaded_size_in_bytes as f64 / duration_seconds as f64) * 100.0;

            format_speed(raw_average_speed)
        } else {
            format_speed(0.0)
        }
    }

    pub fn load(&mut self, size_in_bytes: u64) -> bool {
        if self.is_finished() {
            return false;
        }

        let next_loaded = self.loaded_size_in_bytes + size_in_bytes;

        if next_loaded <= self.total_size_in_bytes {
            self.loaded_size_in_bytes = next_loaded;
        } else {
            self.loaded_size_in_bytes = self.total_size_in_bytes;
        }

        let now = get_now_timestamp();

        // first load need to mark start_time.
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        self.last_loaded_time = Some(now);
        self.instantaneous_speed = self.calculate_instantaneous_speed(size_in_bytes);

        true
    }
}

fn get_now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| {
            println!("Failed to get now time.");
            Duration::default()
        })
        .as_secs()
}

fn format_speed(speed_in_bytes_per_second: f64) -> String {
    const UNITS: [&str; 5] = ["B/s", "KB/s", "MB/s", "GB/s", "TB/s"];
    const THRESHOLD: f64 = 1000.0;

    let mut speed = speed_in_bytes_per_second;
    let mut unit_index = 0;

    while speed >= THRESHOLD && unit_index < UNITS.len() - 1 {
        speed /= THRESHOLD;
        unit_index += 1;
    }

    format!("{:.2} {}", speed, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_now_timestamp() {
        println!("{}", get_now_timestamp());
    }

    #[test]
    fn test_format_speed() {
        assert_eq!(String::from("0.00 B/s"), format_speed(0.0));
        assert_eq!(String::from("800.00 B/s"), format_speed(800.0));
        assert_eq!(String::from("999.99 B/s"), format_speed(999.99));
        assert_eq!(String::from("1.00 KB/s"), format_speed(1000.0));
        assert_eq!(String::from("10.00 KB/s"), format_speed(10000.0));
        assert_eq!(String::from("100.00 KB/s"), format_speed(100000.0));
        assert_eq!(String::from("900.00 KB/s"), format_speed(900000.0));
        assert_eq!(String::from("999.99 KB/s"), format_speed(999990.00));
        assert_eq!(String::from("1000.00 KB/s"), format_speed(999999.99));
        assert_eq!(String::from("1.00 MB/s"), format_speed(1000000.0));
        assert_eq!(String::from("10.00 MB/s"), format_speed(10000000.0));
        assert_eq!(String::from("100.00 MB/s"), format_speed(100000000.0));
        assert_eq!(String::from("900.00 MB/s"), format_speed(900000000.0));
        assert_eq!(String::from("999.99 MB/s"), format_speed(999990000.0));
        assert_eq!(String::from("1000.00 MB/s"), format_speed(999999999.9));
        assert_eq!(String::from("1.00 GB/s"), format_speed(1000000000.0));
        assert_eq!(String::from("10.00 GB/s"), format_speed(10000000000.0));
        assert_eq!(String::from("100.00 GB/s"), format_speed(100000000000.0));
        assert_eq!(String::from("900.00 GB/s"), format_speed(900000000000.0));
        assert_eq!(String::from("999.99 GB/s"), format_speed(999990000000.0));
        assert_eq!(String::from("1000.00 GB/s"), format_speed(999999999999.9));
        assert_eq!(String::from("1.00 TB/s"), format_speed(1000000000000.0));
    }

    // fn test_load() {
    //     let progress_calculator = ProgressCalculator::new(100000);

    //     progress_calculator.load(100);
    // }
}
