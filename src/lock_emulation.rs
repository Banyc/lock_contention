//! # References
//!
//! - blog: <https://preshing.com/20111118/locks-arent-slow-lock-contention-is/>

use std::{
    hint::black_box,
    sync::Mutex,
    time::{Duration, Instant},
};

use rand::Rng;

use crate::poisson_process::duration_until_next_event;

pub fn toggle_lock(
    lock: &Mutex<()>,
    lambda_unlock: f64,
    lambda_lock: f64,
    duration_limit: Duration,
) -> (u64, Duration) {
    let mut tasks_done: u64 = 0;
    let start = Instant::now();
    let mut action = 0;
    let mut rng = rand::thread_rng();
    loop {
        let duration = start.elapsed();
        if duration > duration_limit {
            return (tasks_done, duration);
        }

        match action {
            0 => {
                // Lock then wait until unlock
                let tasks = (duration_until_next_event(lambda_unlock) + 0.5) as usize;
                tasks_done += tasks as u64;
                let _guard = lock.lock().unwrap();
                for _ in 0..tasks {
                    black_box(rng.gen::<usize>());
                }
                action = 1;
            }
            1 => {
                // Unlock then wait until lock
                let tasks = (duration_until_next_event(lambda_lock) + 0.5) as usize;
                tasks_done += tasks as u64;
                for _ in 0..tasks {
                    black_box(rng.gen::<usize>());
                }
                action = 0;
            }
            _ => unreachable!(),
        }
    }
}

pub fn toggle_lock_parallel(
    lock: &Mutex<()>,
    lambda_unlock: f64,
    lambda_lock: f64,
    duration_limit: Duration,
    threads: usize,
) -> Vec<(u64, Duration)> {
    std::thread::scope(|s| {
        let handles = (0..threads)
            .map(|_| s.spawn(|| toggle_lock(lock, lambda_unlock, lambda_lock, duration_limit)))
            .collect::<Vec<_>>();
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn one_thread_never_lock() {
        let lambda_unlock = 1.0 / 1.0; // On average, unlock once every task
        let lambda_lock = 1.0 / 10000000.0; // On average, rarely lock
        let duration_limit = Duration::from_secs(3);
        let lock = Arc::new(Mutex::new(()));

        let (tasks, duration) = toggle_lock(&lock, lambda_unlock, lambda_lock, duration_limit);

        println!("Tasks: {tasks}");
        println!("Duration: {:.02} s", duration.as_secs_f64());
        let tasks_per_sec = tasks as f64 / duration.as_secs_f64();
        println!("Tasks/sec: {:.02}", tasks_per_sec);
    }

    #[test]
    fn one_thread() {
        let lambda_unlock = 1.0 / 2.0; // On average, unlock once every two tasks
        let lambda_lock = 1.0 / 2.0; // On average, lock once every two tasks
        let duration_limit = Duration::from_secs(3);
        let lock = Arc::new(Mutex::new(()));

        let (tasks, duration) = toggle_lock(&lock, lambda_unlock, lambda_lock, duration_limit);

        println!("Tasks: {tasks}");
        println!("Duration: {:.02} s", duration.as_secs_f64());
        let tasks_per_sec = tasks as f64 / duration.as_secs_f64();
        println!("Tasks/sec: {:.02}", tasks_per_sec);
    }

    #[test]
    fn two_threads() {
        let lambda_unlock = 1.0 / 2.0; // On average, unlock once every two tasks
        let lambda_lock = 1.0 / 2.0; // On average, lock once every two tasks
        let duration_limit = Duration::from_secs(3);
        let lock = Arc::new(Mutex::new(()));
        let threads = 2;

        let res = toggle_lock_parallel(&lock, lambda_unlock, lambda_lock, duration_limit, threads);

        for (tasks, duration) in res {
            println!("Tasks: {tasks}");
            println!("Duration: {:.02} s", duration.as_secs_f64());
            let tasks_per_sec = tasks as f64 / duration.as_secs_f64();
            println!("Tasks/sec: {:.02}", tasks_per_sec);
            println!();
        }
    }
}
