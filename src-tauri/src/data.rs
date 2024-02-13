use serde::Serialize;
use ts_rs::TS;


#[derive(Debug, Clone, Copy, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct Proc {
    pub user: f32,
    pub kern: f32,
    pub idle: f32,
}

#[derive(Debug, Clone, Copy, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct SingleCpu {
    // TODO: add total and cpu list
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub idle: f32,
    pub iowait: f32,
    pub irq: f32,
    pub softirq: f32,
    pub steal: f32,
    pub guest: f32,
    pub guest_nice: f32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct CpuStat {
    pub cpu: SingleCpu,
    pub cpus: Vec<SingleCpu>,
}
