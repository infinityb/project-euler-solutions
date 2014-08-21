#[cfg(not(test))]
use std::sync::deque::{BufferPool, Data, Empty, Abort};

type CollatzInt = u64;

#[inline]
fn collatz_next(num: CollatzInt) -> CollatzInt {
    match num % 2 == 0 {
        true => num / 2,
        false => 3 * num + 1
    }
}

#[inline]
fn collatz_length(num: CollatzInt) -> u32 {
    let mut value = num;
    let mut iteration = 1_u32;
    while value != 1 {
        value = collatz_next(value);
        iteration += 1;
    }
    iteration
}

#[cfg(not(test))]
fn main() {
    let (tx, rx) = channel();
    let (worker, stealer) = BufferPool::new().deque();

    for i in range(1 as CollatzInt, 1000000) {
        worker.push(i);
    }

    let jobs = std::os::num_cpus();

    for _ in range(0, jobs) {
        let child_stealer = stealer.clone();
        let finish_tx = tx.clone();

        spawn(proc() {
            let mut longest: (u32, CollatzInt) = (1, 1);
            loop {
                match child_stealer.steal() {
                    Data(num) => {
                        let (len, _) = longest;
                        let new_len = collatz_length(num);
                        if new_len > len {
                            longest = (new_len, num);
                        }
                    },
                    Empty => break,
                    Abort => ()
                }
            }
            finish_tx.send(longest);
        });
    }

    let mut results = Vec::new();
    for _ in range(0, jobs) {
        results.push(rx.recv());
    }

    let mut longest: (CollatzInt, u32) = (1, 1);
    for &(new_len, num) in results.iter() {
        let (_, len) = longest;
        if new_len > len {
            longest = (num, new_len);
        }
    }
    let (len, num) = longest;
    println!("{} with length of {}", len, num);
}


#[test]
fn test_euler_result() {
    assert_eq!(collatz_length(13), 10);
}
