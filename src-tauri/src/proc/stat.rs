use std::{char::ParseCharError, fs, num::ParseIntError};

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(super) struct ProcStat {
    pub(super) pid: isize,
    pub(super) comm: String,
    pub(super) state: char,
    pub(super) ppid: isize,
    pub(super) pgrp: isize,
    pub(super) session: isize,
    pub(super) tty_nr: isize,
    pub(super) tpgid: isize,
    pub(super) flags: usize,
    pub(super) minflt: usize,
    pub(super) cminflt: usize,
    pub(super) majflt: usize,
    pub(super) cmajflt: usize,
    pub(super) utime: usize,
    pub(super) stime: usize,
    pub(super) cutime: isize,
    pub(super) cstime: isize,
    pub(super) priority: isize,
    pub(super) nice: isize,
    pub(super) num_threads: isize,
    pub(super) itrealvalue: isize,
    pub(super) starttime: usize,
    pub(super) vsize: usize,
    pub(super) rss: isize,
    pub(super) rsslim: usize,
    pub(super) startcode: usize,
    pub(super) endcode: usize,
    pub(super) startstack: usize,
    pub(super) kstkesp: usize,
    pub(super) kstkeip: usize,
    pub(super) signal: usize,
    pub(super) blocked: usize,
    pub(super) sigignore: usize,
    pub(super) sigcatch: usize,
    pub(super) wchan: usize,
    pub(super) nswap: usize,
    pub(super) cnswap: usize,
    pub(super) exit_signal: isize,
    pub(super) processor: isize,
    pub(super) rt_priority: usize,
    pub(super) policy: usize,
    pub(super) delayacct_blkio_ticks: usize,
    pub(super) guest_time: usize,
    pub(super) cguest_time: isize,
    pub(super) start_data: usize,
    pub(super) end_data: usize,
    pub(super) start_brk: usize,
    pub(super) arg_start: usize,
    pub(super) arg_end: usize,
    pub(super) env_start: usize,
    pub(super) env_end: usize,
    pub(super) exit_code: isize,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum StateError {
    #[error("Proc[pid] not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ParseIntError> for StateError {
    fn from(value: ParseIntError) -> Self {
        StateError::Other(value.into())
    }
}

impl From<ParseCharError> for StateError {
    fn from(value: ParseCharError) -> Self {
        StateError::Other(value.into())
    }
}

impl ProcStat {
    pub(super) fn new(pid: usize) -> Result<Self, StateError> {
        let path = format!("/proc/{}/stat", pid);

        let string = fs::read_to_string(path).map_err(|e| {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                StateError::NotFound
            } else {
                StateError::Other(e.into())
            }
        })?;

        let (before, comm, after) = {
            let (before, mid) = string.split_once("(").expect("/proc should contain (");
            let after = mid.split(")").last().expect("/proc should contain )");
            let mid = mid.split(")").take(mid.len() - 1).join("");

            (before, mid, after)
        };

        let parsed = before
            .split_ascii_whitespace()
            .chain([&comm as &str])
            .chain(after.split_ascii_whitespace())
            .map(str::to_string)
            .collect_vec();

        Ok(Self {
            pid: parsed[0].parse()?,
            comm: parsed[1].clone(),
            state: parsed[2].parse()?,
            ppid: parsed[3].parse()?,
            pgrp: parsed[4].parse()?,
            session: parsed[5].parse()?,
            tty_nr: parsed[6].parse()?,
            tpgid: parsed[7].parse()?,
            flags: parsed[8].parse()?,
            minflt: parsed[9].parse()?,
            cminflt: parsed[10].parse()?,
            majflt: parsed[11].parse()?,
            cmajflt: parsed[12].parse()?,
            utime: parsed[13].parse()?,
            stime: parsed[14].parse()?,
            cutime: parsed[15].parse()?,
            cstime: parsed[16].parse()?,
            priority: parsed[17].parse()?,
            nice: parsed[18].parse()?,
            num_threads: parsed[19].parse()?,
            itrealvalue: parsed[20].parse()?,
            starttime: parsed[21].parse()?,
            vsize: parsed[22].parse()?,
            rss: parsed[23].parse()?,
            rsslim: parsed[24].parse()?,
            startcode: parsed[25].parse()?,
            endcode: parsed[26].parse()?,
            startstack: parsed[27].parse()?,
            kstkesp: parsed[28].parse()?,
            kstkeip: parsed[29].parse()?,
            signal: parsed[30].parse()?,
            blocked: parsed[31].parse()?,
            sigignore: parsed[32].parse()?,
            sigcatch: parsed[33].parse()?,
            wchan: parsed[34].parse()?,
            nswap: parsed[35].parse()?,
            cnswap: parsed[36].parse()?,
            exit_signal: parsed[37].parse()?,
            processor: parsed[38].parse()?,
            rt_priority: parsed[39].parse()?,
            policy: parsed[40].parse()?,
            delayacct_blkio_ticks: parsed[41].parse()?,
            guest_time: parsed[42].parse()?,
            cguest_time: parsed[43].parse()?,
            start_data: parsed[44].parse()?,
            end_data: parsed[45].parse()?,
            start_brk: parsed[46].parse()?,
            arg_start: parsed[47].parse()?,
            arg_end: parsed[48].parse()?,
            env_start: parsed[49].parse()?,
            env_end: parsed[50].parse()?,
            exit_code: parsed[51].parse()?,
        })
    }

    // pub(super) fn total_time(&self) -> usize {
    //     // self.utime
    //     //     + self.stime
    //     //     + usize::try_from(self.cutime).expect("cutime was negative")
    //     //     + usize::try_from(self.cstime).expect("cstime was negative")
    //     self.starttime
    // }
}
