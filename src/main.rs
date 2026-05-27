use std::{fs, thread::sleep, time::Duration};

fn main() {
    loop {
        dbg!(
            fs::read_to_string("/sys/class/power_supply/ACAD/online")
                .unwrap()
                .trim_end()
        );
        sleep(Duration::from_secs(10));
    }
}
