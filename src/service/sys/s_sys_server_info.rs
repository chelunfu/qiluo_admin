use crate::model::sys::args::aserve_info::*;
use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::result::Result;
use std::{convert::Infallible, time::Duration};
use sysinfo::{Networks, System};
use tokio_stream::StreamExt as _;

pub async fn server_event() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(move || {
        let r = get_oper_sys_info();
        let str = serde_json::to_string(&r).unwrap_or_else(|_| "0".to_string());
        Event::default().data(str.clone())
    })
    .map(Ok)
    .throttle(Duration::from_secs(2));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(2))
            .text("keep-alive-text"),
    )
}

fn get_oper_sys_info() -> SysInfo {
    let mut sys: System = System::new_all();
    sys.refresh_all();
    let pid = sysinfo::get_current_pid().expect("failed to get PID");
    let server = Server {
        oper_sys_name: System::name().unwrap_or_else(|| "unknown".to_owned()),
        host_name: System::host_name().unwrap_or_else(|| "unknown".to_owned()),
        system_version: System::long_os_version().unwrap_or_else(|| "unknown".to_owned()),
        system_kerne: System::kernel_version().unwrap_or_else(|| "unknown".to_owned()),
    };
    let process = match sys.process(pid) {
        Some(p) => Process {
            name: p.name().to_string(),
            used_memory: p.memory(),
            used_virtual_memory: p.virtual_memory(),
            cup_usage: p.cpu_usage(),
            start_time: p.start_time(),
            run_time: p.run_time(),
            disk_usage: DiskUsage {
                read_bytes: p.disk_usage().read_bytes,
                total_read_bytes: p.disk_usage().total_read_bytes,
                written_bytes: p.disk_usage().written_bytes,
                total_written_bytes: p.disk_usage().total_written_bytes,
            },
        },
        None => Process {
            ..Default::default()
        },
    };

    let mut network: Vec<Network> = Vec::new();
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        network.push(Network {
            name: interface_name.to_string(),
            received: data.received(),
            total_received: data.total_received(),
            transmitted: data.transmitted(),
            total_transmitted: data.total_transmitted(),
        });
    }
    let cpu = Cpu {
        name: sys.cpus()[0].brand().to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cores: sys
            .physical_core_count()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Unknown".to_owned()),
        total_use: sys.global_cpu_info().cpu_usage(),
        frequency: sys.global_cpu_info().frequency(),
        processors: sys.cpus().len(),
    };
    let load_avg = System::load_average();
    let cpu_load = CpuLoad {
        one: load_avg.one,
        five: load_avg.five,
        fifteen: load_avg.fifteen,
    };
    let memory = Memory {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };
    SysInfo {
        server,
        cpu,
        memory,
        process,
        network,
        cpu_load,
    }
}
