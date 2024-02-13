use anyhow::Context;
use std::fs;

#[derive(Debug, Clone)]
pub(super) struct SysStat {
    pub(super) cpu: Cpu,
    pub(super) cpus: Vec<Cpu>,
}

impl SysStat {
    pub(super) fn new() -> Result<Self, anyhow::Error> {
        let stat = fs::read_to_string("/proc/stat")?;

        let (cpu, cpus) = cpu_stat(stat.clone())?;

        Ok(Self { cpu, cpus })
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Cpu {
    pub(super) user: usize,
    pub(super) nice: usize,
    pub(super) system: usize,
    pub(super) idle: usize,
    pub(super) iowait: usize,
    pub(super) irq: usize,
    pub(super) softirq: usize,
    pub(super) steal: usize,
    pub(super) guest: usize,
    pub(super) guest_nice: usize,
}

impl Cpu {
    fn new(cpu: &[usize]) -> Self {
        if cpu.len() != 10 {
            panic!("Cpu line must be long 10!");
        }

        Self {
            user: cpu[0],
            nice: cpu[1],
            system: cpu[2],
            idle: cpu[3],
            iowait: cpu[4],
            irq: cpu[5],
            softirq: cpu[6],
            steal: cpu[7],
            guest: cpu[8],
            guest_nice: cpu[9],
        }
    }

    pub(super) fn uptime(&self) -> usize {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
            + self.guest
            + self.guest_nice
    }
}

fn cpu_stat(stat: String) -> Result<(Cpu, Vec<Cpu>), anyhow::Error> {
    let cpus = stat
        .lines()
        .filter(|l| l.contains("cpu"))
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .context("/proc/stat cpu line didn't contain a number")?
        .iter()
        .map(|v| Cpu::new(v))
        .collect::<Vec<_>>();

    let total = cpus[0];
    let cpus = cpus[1..].to_vec();

    Ok((total, cpus))
}
