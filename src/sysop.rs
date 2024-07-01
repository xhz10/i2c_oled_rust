use std::thread;

use systemstat::{saturating_sub_bytes, Duration, IpAddr, Platform, System};

pub struct SystemOperation {
    plat: System,
}

impl SystemOperation {
    // new的时候使用的
    pub fn new() -> SystemOperation {
        Self {
            plat: System::new(),
        }
    }

    // 获取ip地址
    pub fn ip_addr(&self) -> String {
        match self.plat.networks().ok().and_then(|networks| {
            networks.values()
                .flat_map(|na| &na.addrs)
                .filter_map(|addr| {
                    if let IpAddr::V4(ip) = addr.addr {
                        let s_addr = ip.to_string();
                        if s_addr.starts_with("192") {
                            return Some(s_addr);
                        }
                    }
                    None
                })
                .next()  // 获取第一个符合条件的 IP 地址
        }) {
            Some(val) =>{
                val
            }
            None => {
                String::new()
            }
        }
    }

    pub fn cpu_info(&self) -> u32 {
        match self.plat.cpu_load_aggregate()
            .and_then(|cla| {
            thread::sleep(Duration::from_millis(500));
            cla.done()
        }) {
            Ok(cpu) => {
                return ((cpu.user + cpu.system + cpu.nice) * 100.0).round() as u32;
            }
            Err(_) => {
                0
            }
        }
    }

    // 返回内存利用率
    pub fn memory_info(&self) -> u32 {
        match self.plat.memory() {
            Ok(memory) => {
                ((
                    saturating_sub_bytes(memory.total, memory.free).0 as f32
                        * 100_f32) / memory.total.0 as f32)
                    .round() as u32
            }
            Err(_) => { 0 }
        }
    }

    // 获取CPU的温度
    pub fn cpu_temperature(&self) -> u32 {
        match self.plat.cpu_temp() {
            Ok(t) => {
                t as u32
            }
            Err(_) => 0
        }
    }
}
