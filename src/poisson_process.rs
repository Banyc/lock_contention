//! # References
//!
//! - blog: <https://preshing.com/20111007/how-to-generate-random-timings-for-a-poisson-process/>

use std::f64::consts::E;

use rand::Rng;

pub fn rate(events: f64, time_duration: f64) -> f64 {
    events / time_duration
}

pub fn prob_of_one_event_within_next(duration: f64, lambda: f64) -> f64 {
    // CDF of Exponential distribution
    1. - E.powf(-lambda * duration)
}

pub fn duration_until_next_event(lambda: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let uniform_rv: f64 = 1. - rng.gen_range(0. ..1.); // (0, 1]
    -(uniform_rv.ln()) / lambda
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn prob_of_next_minute() {
        let lambda = rate(1., Duration::from_secs(40 * 60).as_secs_f64());
        let prob = prob_of_one_event_within_next(Duration::from_secs(60).as_secs_f64(), lambda);
        assert!((prob - 0.0247).abs() < 0.0001);
    }

    #[test]
    fn prob_of_next_10_minutes() {
        let lambda = rate(1., Duration::from_secs(40 * 60).as_secs_f64());
        let prob =
            prob_of_one_event_within_next(Duration::from_secs(10 * 60).as_secs_f64(), lambda);
        assert!((prob - 0.221).abs() < 0.001);
    }

    #[test]
    fn prob_of_next_40_minutes() {
        let lambda = rate(1., Duration::from_secs(40 * 60).as_secs_f64());
        let prob =
            prob_of_one_event_within_next(Duration::from_secs(40 * 60).as_secs_f64(), lambda);
        assert!((prob - 0.632).abs() < 0.001);
    }

    #[test]
    fn prob_of_next_64_38_minutes() {
        let lambda = rate(1., Duration::from_secs(40 * 60).as_secs_f64());
        let prob = prob_of_one_event_within_next(
            Duration::from_secs_f64(64.38 * 60.).as_secs_f64(),
            lambda,
        );
        assert!((prob - 0.8).abs() < 0.001);
    }

    #[test]
    fn next_event() {
        let lambda = rate(1., Duration::from_secs(40 * 60).as_secs_f64());
        let events = 128;
        let mut whole_duration = 0.;
        for _ in 0..events {
            let duration = duration_until_next_event(lambda);
            whole_duration += duration;
            println!("{:.01} min", duration / 60.);
        }
        assert!((events as f64 / whole_duration - lambda) < 0.01);
    }
}
