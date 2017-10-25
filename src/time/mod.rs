use std::time::Duration;

pub trait AsMilliseconds {
    fn num_milliseconds(&self) -> u64;
}

impl AsMilliseconds for Duration {
    fn num_milliseconds(&self) -> u64 {
        let secs_part = self.as_secs() * 1000_u64;
        let nanos_part = self.subsec_nanos() / 1000_1000_u32;
        secs_part + nanos_part as u64
    }
}