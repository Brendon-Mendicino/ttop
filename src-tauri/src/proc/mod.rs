use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Context;

use crate::{data, sys};

use self::stat::{ProcStat, StateError};

mod stat;

#[derive(Debug, Clone)]
pub struct Procs {
    pids: HashSet<usize>,
    procs: HashMap<usize, SysProc>,
}

#[derive(Debug, Clone)]
pub struct SysProc {
    pub pid: usize,
    stat: ProcStat,
    old_stat: ProcStat,
}

impl SysProc {
    fn new(pid: usize) -> Result<Self, StateError> {
        let stat = ProcStat::new(pid)?;

        Ok(Self {
            pid,
            old_stat: stat.clone(),
            stat,
        })
    }

    fn update(&mut self) -> Result<(), StateError> {
        let stat = ProcStat::new(self.pid)?;

        self.old_stat = self.stat.clone();
        self.stat = stat;

        Ok(())
    }

    pub fn cpu_stat(&self, sys: &sys::Sys) -> data::Proc {
        let uptime_diff = (sys.uptime - sys.old_uptime).max(1) as f32;
        // let diff = self.stat.total_time() - self.old_stat.total_time();
        let a = &self.stat;
        let b = &self.old_stat;

        let user = ((a.utime - b.utime) as f32 * 100. / uptime_diff).clamp(0., 100.);
        let kern = ((a.stime - b.stime) as f32 * 100. / uptime_diff).clamp(0., 100.);

        data::Proc {
            user,
            kern,
            idle: 100. - user - kern,
        }
    }
}

impl Procs {
    pub fn new() -> Result<Self, anyhow::Error> {
        let pids = get_pids()?;

        // Filter gone processes
        let procs = pids
            .iter()
            .filter_map(|&pid| match SysProc::new(pid) {
                Err(StateError::NotFound) => None,
                Err(StateError::Other(e)) => Some(Err(e)),
                Ok(p) => Some(Ok((pid, p))),
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { pids, procs })
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        let pids = get_pids()?;

        // Get pid difference
        let diff = self.pids.difference(&pids).cloned().collect::<Vec<_>>();
        let new = pids.difference(&self.pids).cloned().collect::<Vec<_>>();

        diff.iter().for_each(|pid| {
            self.pids.remove(pid);
            self.procs.remove(pid);
        });

        // Update current procs
        for proc in self.procs.values_mut() {
            match proc.update() {
                Err(StateError::Other(e)) => return Err(e),
                _ => (),
            }
        }

        // Add new procs
        let new_procs = new
            .into_iter()
            .filter_map(|pid| match SysProc::new(pid) {
                Err(e) if matches!(e, StateError::NotFound) => None,
                Err(e) => Some(Err(e)),
                Ok(p) => Some(Ok((pid, p))),
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        self.procs.extend(new_procs);
        self.pids = pids;

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &SysProc> {
        self.procs.values()
    }
}

fn get_pids() -> Result<HashSet<usize>, anyhow::Error> {
    Ok(fs::read_dir("/proc")
        .context("Could not open /proc")?
        .into_iter()
        .filter_map(|f| match f {
            Ok(entry) if entry.path().is_dir() => Some(entry.file_name()),
            _ => None,
        })
        .filter_map(|dir| dir.to_str().map(str::to_string))
        .filter_map(|dir| dir.parse().ok())
        .collect())
}
