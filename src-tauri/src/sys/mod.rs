use anyhow;
use itertools::Itertools;

use crate::{data, proc};

mod stat;

#[derive(Debug, Clone)]
pub struct Sys {
    stat: stat::SysStat,
    old_stat: stat::SysStat,
    pub uptime: usize,
    pub old_uptime: usize,
    procs: proc::Procs,
}

impl Sys {
    pub fn new() -> Result<Self, anyhow::Error> {
        let stat = stat::SysStat::new()?;
        let procs = proc::Procs::new()?;

        Ok(Self {
            stat: stat.clone(),
            old_stat: stat.clone(),
            uptime: stat.cpu.uptime(),
            old_uptime: stat.cpu.uptime(),
            procs,
        })
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        std::mem::swap(&mut self.stat, &mut self.old_stat);
        self.stat = stat::SysStat::new()?;

        self.old_uptime = self.uptime;
        self.uptime = self.stat.cpu.uptime();

        self.procs.update()?;

        Ok(())
    }
}

impl From<&Sys> for data::CpuStat {
    fn from(value: &Sys) -> Self {
        let new = value.stat.cpu;
        let old = value.old_stat.cpu;
        let diff = (new.uptime() - old.uptime()).max(1) as f32;

        let calc_stat = |a: &stat::Cpu, b: &stat::Cpu| data::SingleCpu {
            user: (a.user - b.user) as f32 * 100.0 / diff,
            nice: (a.nice - b.nice) as f32 * 100.0 / diff,
            system: (a.system - b.system) as f32 * 100.0 / diff,
            idle: (a.idle - b.idle) as f32 * 100.0 / diff,
            iowait: (a.iowait - b.iowait) as f32 * 100.0 / diff,
            irq: (a.irq - b.irq) as f32 * 100.0 / diff,
            softirq: (a.softirq - b.softirq) as f32 * 100.0 / diff,
            steal: (a.steal - b.steal) as f32 * 100.0 / diff,
            guest: (a.guest - b.guest) as f32 * 100.0 / diff,
            guest_nice: (a.guest_nice - b.guest_nice) as f32 * 100.0 / diff,
        };

        let cpu = calc_stat(&new, &old);

        let cpus = value
            .stat
            .cpus
            .iter()
            .zip(value.old_stat.cpus.iter())
            .map(|(new, old)| calc_stat(new, old))
            .collect_vec();

        data::CpuStat { cpu, cpus }
    }
}

impl From<&Sys> for Vec<data::Proc> {
    fn from(value: &Sys) -> Self {
        value
            .procs
            .iter()
            .sorted_by_key(|p| p.pid)
            .map(|p| p.cpu_stat(value))
            .collect()
    }
}
