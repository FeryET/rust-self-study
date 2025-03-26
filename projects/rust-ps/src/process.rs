use std::str::FromStr;

use crate::error;
use strum::EnumString;

#[derive(Debug)]
pub struct ProcessInfo {
    pub id: u32,
}
#[derive(Debug)]
pub struct ProcessStats {
    pub pid: i32,                    // (1) Process ID
    pub command: String,             // (2) Filename of the executable
    pub state: State,                // (3) Process state
    pub ppid: i32,                   // (4) Parent process ID
    pub pgrp: i32,                   // (5) Process group ID
    pub session: i32,                // (6) Session ID
    pub tty_nr: i32,                 // (7) Controlling terminal
    pub tpgid: i32,                  // (8) Foreground process group ID
    pub flags: u64,                  // (9) Kernel flags
    pub minflt: u64,                 // (10) Minor faults
    pub cminflt: u64,                // (11) Minor faults by children
    pub majflt: u64,                 // (12) Major faults
    pub cmajflt: u64,                // (13) Major faults by children
    pub utime: u64,                  // (14) User mode time (in clock ticks)
    pub stime: u64,                  // (15) Kernel mode time (in clock ticks)
    pub cutime: i64,                 // (16) User mode time by children (in clock ticks)
    pub cstime: i64,                 // (17) Kernel mode time by children (in clock ticks)
    pub priority: i64,               // (18) Priority
    pub nice: i64,                   // (19) Nice value
    pub num_threads: i64,            // (20) Number of threads
    pub itrealvalue: i64,            // (21) Time before next SIGALRM
    pub starttime: u128,             // (22) Start time after system boot (in clock ticks)
    pub vsize: u64,                  // (23) Virtual memory size (in bytes)
    pub rss: i64,                    // (24) Resident Set Size
    pub rsslim: u64,                 // (25) RSS limit (in bytes)
    pub startcode: u64,              // (26) Start address of program text
    pub endcode: u64,                // (27) End address of program text
    pub startstack: u64,             // (28) Start address of stack
    pub kstkesp: u64,                // (29) Current value of ESP (stack pointer)
    pub kstkeip: u64,                // (30) Current value of EIP (instruction pointer)
    pub signal: u64,                 // (31) Bitmap of pending signals
    pub blocked: u64,                // (32) Bitmap of blocked signals
    pub sigignore: u64,              // (33) Bitmap of ignored signals
    pub sigcatch: u64,               // (34) Bitmap of caught signals
    pub wchan: u64,                  // (35) Waiting channel
    pub nswap: u64,                  // (36) Number of pages swapped
    pub cnswap: u64,                 // (37) Cumulative nswap for children
    pub exit_signal: i32,            // (38) Signal sent to parent on exit
    pub processor: i32,              // (39) CPU number last executed on
    pub rt_priority: u32,            // (40) Real-time scheduling priority
    pub policy: u32,                 // (41) Scheduling policy
    pub delayacct_blkio_ticks: u128, // (42) Block I/O delays (in clock ticks)
    pub guest_time: u64,             // (43) Guest time (in clock ticks)
    pub cguest_time: i64,            // (44) Guest time by children (in clock ticks)
    pub start_data: u64,             // (45) Start address of program data
    pub end_data: u64,               // (46) End address of program data
    pub start_brk: u64,              // (47) Start address of heap
    pub arg_start: u64,              // (48) Start address of command-line arguments
    pub arg_end: u64,                // (49) End address of command-line arguments
    pub env_start: u64,              // (50) Start address of environment
    pub env_end: u64,                // (51) End address of environment
    pub exit_code: i32,              // (52) Exit status
}
#[derive(EnumString, Debug, PartialEq)]
enum State {
    #[strum(serialize = "R")]
    Running,
    #[strum(serialize = "S")]
    Sleeping,
    #[strum(serialize = "D")]
    DiskSleep,
    #[strum(serialize = "Z")]
    Zombie,
    #[strum(serialize = "T")]
    Stopped,
    #[strum(serialize = "t")]
    TracingStop,
    #[strum(serialize = "W")]
    Paging,
    #[strum(serialize = "X", serialize = "x")]
    Dead,
    #[strum(serialize = "K")]
    WakeKill,
    #[strum(serialize = "W")]
    Waking,
    #[strum(serialize = "P")]
    Parked,
    #[strum(serialize = "I")]
    Idle,
}

impl FromStr for ProcessStats {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 52 {
            return Err(error::Error::ParseError(
                "mismatch in the number of fields required in stats file".into(),
            ));
        }

        Ok(ProcessStats {
            pid: parts[0]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid pid".into()))?,
            command: parts[1][1..parts[1].len() - 1].into(), // Remove parentheses
            state: parts[2]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid state".into()))?,
            ppid: parts[3]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid ppid".into()))?,
            pgrp: parts[4]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid pgrp".into()))?,
            session: parts[5]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid session".into()))?,
            tty_nr: parts[6]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid tty_nr".into()))?,
            tpgid: parts[7]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid tpgid".into()))?,
            flags: parts[8]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid flags".into()))?,
            minflt: parts[9]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid minflt".into()))?,
            cminflt: parts[10]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cminflt".into()))?,
            majflt: parts[11]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid majflt".into()))?,
            cmajflt: parts[12]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cmajflt".into()))?,
            utime: parts[13]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid utime".into()))?,
            stime: parts[14]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid stime".into()))?,
            cutime: parts[15]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cutime".into()))?,
            cstime: parts[16]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cstime".into()))?,
            priority: parts[17]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid priority".into()))?,
            nice: parts[18]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid nice".into()))?,
            num_threads: parts[19]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid num_threads".into()))?,
            itrealvalue: parts[20]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid itrealvalue".into()))?,
            starttime: parts[21]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid starttime".into()))?,
            vsize: parts[22]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid vsize".into()))?,
            rss: parts[23]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid rss".into()))?,
            rsslim: parts[24]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid rsslim".into()))?,
            startcode: parts[25]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid startcode".into()))?,
            endcode: parts[26]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid endcode".into()))?,
            startstack: parts[27]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid startstack".into()))?,
            kstkesp: parts[28]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid kstkesp".into()))?,
            kstkeip: parts[29]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid kstkeip".into()))?,
            signal: parts[30]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid signal".into()))?,
            blocked: parts[31]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid blocked".into()))?,
            sigignore: parts[32]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid sigignore".into()))?,
            sigcatch: parts[33]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid sigcatch".into()))?,
            wchan: parts[34]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid wchan".into()))?,
            nswap: parts[35]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid nswap".into()))?,
            cnswap: parts[36]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cnswap".into()))?,
            exit_signal: parts[37]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid exit_signal".into()))?,
            processor: parts[38]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid processor".into()))?,
            rt_priority: parts[39]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid rt_priority".into()))?,
            policy: parts[40]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid policy".into()))?,
            delayacct_blkio_ticks: parts[41]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid delayacct_blkio_ticks".into()))?,
            guest_time: parts[42]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid guest_time".into()))?,
            cguest_time: parts[43]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid cguest_time".into()))?,
            start_data: parts[44]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid start_data".into()))?,
            end_data: parts[45]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid end_data".into()))?,
            start_brk: parts[46]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid start_brk".into()))?,
            arg_start: parts[47]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid arg_start".into()))?,
            arg_end: parts[48]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid arg_end".into()))?,
            env_start: parts[49]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid env_start".into()))?,
            env_end: parts[50]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid env_end".into()))?,
            exit_code: parts[51]
                .parse()
                .map_err(|_| error::Error::ParseError("Invalid exit_code".into()))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_stat_parsing() {
        let r = "59 (migration/11) S 2 0 0 0 -1 69238848 0 \
                       0 0 0 0 194 0 0 -100 0 1 0 7 0 0 \
                       18446744073709551615 0 0 0 0 0 0 0 2147483647 \
                       0 0 0 0 17 11 99 1 0 0 0 0 0 0 0 0 \
                       0 0 0"
            .trim();
        let p = ProcessStats::from_str(r);
        assert!(p.is_ok());
        let p = p.unwrap();

        assert_eq!(p.pid, 59);
        assert_eq!(p.command, "migration/11");
        assert_eq!(p.state, State::Sleeping);
        assert_eq!(p.ppid, 2);
        assert_eq!(p.pgrp, 0);
        assert_eq!(p.session, 0);
        assert_eq!(p.tty_nr, 0);
        assert_eq!(p.tpgid, -1);
        assert_eq!(p.flags, 69238848);
        assert_eq!(p.minflt, 0);
        assert_eq!(p.cminflt, 0);
        assert_eq!(p.majflt, 0);
        assert_eq!(p.cmajflt, 0);
        assert_eq!(p.utime, 0);
        assert_eq!(p.stime, 194);
        assert_eq!(p.cutime, 0);
        assert_eq!(p.cstime, 0);
        assert_eq!(p.priority, -100);
        assert_eq!(p.nice, 0);
        assert_eq!(p.num_threads, 1);
        assert_eq!(p.itrealvalue, 0);
        assert_eq!(p.starttime, 7);
        assert_eq!(p.vsize, 0);
        assert_eq!(p.rss, 0);
        assert_eq!(p.rsslim, 18446744073709551615);
        assert_eq!(p.startcode, 0);
        assert_eq!(p.endcode, 0);
        assert_eq!(p.startstack, 0);
        assert_eq!(p.kstkesp, 0);
        assert_eq!(p.kstkeip, 0);
        assert_eq!(p.signal, 0);
        assert_eq!(p.blocked, 0);
        assert_eq!(p.sigignore, 2147483647);
        assert_eq!(p.sigcatch, 0);
        assert_eq!(p.wchan, 0);
        assert_eq!(p.nswap, 0);
        assert_eq!(p.cnswap, 0);
        assert_eq!(p.exit_signal, 17);
        assert_eq!(p.processor, 11);
        assert_eq!(p.rt_priority, 99);
        assert_eq!(p.policy, 1);
        assert_eq!(p.delayacct_blkio_ticks, 0);
        assert_eq!(p.guest_time, 0);
        assert_eq!(p.cguest_time, 0);
        assert_eq!(p.start_data, 0);
        assert_eq!(p.end_data, 0);
        assert_eq!(p.start_brk, 0);
        assert_eq!(p.arg_start, 0);
        assert_eq!(p.arg_end, 0);
        assert_eq!(p.env_start, 0);
        assert_eq!(p.env_end, 0);
        assert_eq!(p.exit_code, 0);
    }

    #[test]
    fn test_process_stat_parsing_failure() {
        let too_many_fields = "59 (migration/11) S 2 0 0 0 -1 69238848 0 \
        0 0 0 0 194 0 0 -100 0 1 0 7 0 0 \
        18446744073709551615 0 0 0 0 0 0 0 2147483647 \
        0 0 0 0 17 11 99 1 0 0 0 0 0 0 0 0 \
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
            .trim();
        let p = ProcessStats::from_str(too_many_fields);
        assert!(p.is_err());
    }
}
