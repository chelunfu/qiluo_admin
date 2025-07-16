use std::{
    hint::spin_loop,
    sync::atomic::{AtomicI64, Ordering},
    time::SystemTime,
}; 

// const DIFFERENCE: u64 = CUSTOM_EPOCH - TARGET_EPOCH;
pub struct SnowflakeIdGenerator {
    pub machine_id: i32,
    pub node_id: i32,
    sequence: AtomicI64,
    last_timestamp: AtomicI64,
    unix_epoch: SystemTime,
}

impl SnowflakeIdGenerator {
    pub fn new(machine_id: i32, node_id: i32) -> Self {
        let sequence = AtomicI64::new(0);
        let last_timestamp = AtomicI64::new(0);

        let unix_epoch = std::time::UNIX_EPOCH;

        SnowflakeIdGenerator {
            machine_id,
            sequence,
            node_id,
            last_timestamp,
            unix_epoch,
        }
    }

    pub fn real_time_generate(&self) -> i64 {
        let current_timestamp = self.get_current_timestamp();
        // 如果时间戳小于上一次生成的时间戳，则说明时钟回退，这里可以选择等待或者报错处理
        if current_timestamp < self.last_timestamp.load(Ordering::Relaxed) {
            panic!("时钟回退错误");
        }

        if current_timestamp == self.last_timestamp.load(Ordering::Relaxed) {
            // 同一时间戳下，递增序列号
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
            if sequence >= 4096 {
                // 序列号超出范围，等待下一毫秒
                loop {
                    if self.get_current_timestamp() > current_timestamp {
                        break;
                    }
                    spin_loop();
                }
                return self.real_time_generate();
            }
        } else {
            // 新的时间戳，重置序列号
            self.sequence.store(0, Ordering::SeqCst);
        }

        self.last_timestamp
            .store(current_timestamp, Ordering::Relaxed);
        let mut id = current_timestamp;

        id <<= 22;
        id |= (self.machine_id as i64) << 12;
        id |= (self.node_id as i64) << 10;
        id |= self.sequence.load(Ordering::Relaxed);
        id
    }

    fn get_current_timestamp(&self) -> i64 {
        std::time::SystemTime::now()
            .duration_since(self.unix_epoch)
            .unwrap()
            .as_millis() as i64
    }
}
