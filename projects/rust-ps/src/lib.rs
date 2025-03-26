use std::{fs::File, io::Read, str::FromStr};

use error::Error;
use process::{ProcessInfo, ProcessStats};

mod error;
mod process;

pub fn parse_pid_status_file(process_info: &ProcessInfo) -> Result<ProcessStats, Error> {
    let mut content: String = String::new();
    let _ = File::open(format!("/proc/{}/stat", process_info.id))
        .map_err(|e| Error::CannotReadStatusFile(format!("{}", e)))?
        .read_to_string(&mut content)
        .map_err(|e| Error::CannotReadStatusFile(format!("{}", e)))?;
    ProcessStats::from_str(content.as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::seq::IndexedRandom;
    use rand::{Rng, SeedableRng};
    use std::fs::read_dir;

    fn get_random_process_stat_pid() -> u32 {
        // Collect all numeric directory names (PIDs) from /proc
        let pids: Vec<u32> = read_dir("/proc")
            .expect("Failed to read /proc")
            .filter_map(|entry| {
                let entry = entry.expect("Failed to read dir entry");
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();

                // Check if directory and numeric name
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                    && name.chars().all(|c| c.is_ascii_digit())
                {
                    name.parse::<u32>().ok()
                } else {
                    None
                }
            })
            .collect();

        if pids.is_empty() {
            panic!("No valid PIDs found in /proc");
        }

        // Create seeded RNG for deterministic testing
        let mut rng = StdRng::seed_from_u64(42);
        pids.choose(&mut rng).unwrap().clone()
    }

    #[test]
    fn test_process_stat_parsing() {
        let pid = get_random_process_stat_pid();
        let process_info = ProcessInfo { id: pid };
        let stats = parse_pid_status_file(&process_info);
        assert!(stats.is_ok());
        let stats = stats.unwrap();
        assert_eq!(stats.pid as i64, pid as i64);
    }
}
