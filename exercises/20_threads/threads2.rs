// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::{Arc,Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        // Arc::clone(&status)克隆Arc，增加饮用记数
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // status_shared.lock().unwrap()获取Mutex的锁，返回一个可变饮用，允许修改JobStatus。
            let mut status=status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // 再次锁定Mutex以获取JobStatus的最终值。
    let final_status=status.lock().unwrap();
    println!("Jobs done: {}",final_status.jobs_done);
}
