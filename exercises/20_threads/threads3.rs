use std::{sync::mpsc, thread, time::Duration};

/*
要解决文件中的问题，我们需要确保 tx（发送者）可以在两个线程中使用。
由于 mpsc::Sender 实现了 Clone 特性，我们可以克隆 tx 以便在多个线程中共享它。
以下是具体的解决方案和详细解释：

解决方案
1. 克隆 tx 发送者:
    使用 tx.clone() 方法克隆 tx 发送者。
    将原始 tx 发送者传递给第一个线程，将克隆的 tx 发送者传递给第二个线程。

2. 修改 send_tx 函数:
    在 send_tx 函数中，克隆 tx 发送者。
    将原始 tx 发送者传递给第一个线程。
    将克隆的 tx 发送者传递给第二个线程。
*/

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // 克隆tx发送者
    let tx_clone=tx.clone();
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // 第二个线程使用克隆的tx发送者
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
