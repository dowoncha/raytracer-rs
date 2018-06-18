mod task;
use self::task::{DeviceTask, DeviceTaskKind};

mod device_cpu;

enum DeviceKind {
    None,
    CPU,
    Network,
    Multi
}

struct DeviceInfo {
    kind: DeviceKind,
    description: String,
    id: String,
    num: i32,

    /* Features */
    has_osl: bool,
    cpu_threads: i32,
    multi_devices: Vec<DeviceInfo>
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            kind: DeviceKind::CPU,
            id: "CPU".to_string(),
            description: String::new(),
            num: 0,
            has_osl: false,
            cpu_threads: 0,
            multi_devices: Vec::new()
        }
    }

}

trait Device {
    fn new(info: &DeviceInfo, background: bool) -> Self;

    // fn get_split_task_count(task: &DeviceTask);
    fn task_add(&self, task: &DeviceTask);
    fn task_wait(&self);
    fn task_cancel(&self);
}

