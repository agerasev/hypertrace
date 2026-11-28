use std::time::Duration;

use wgame::app::time::Instant;

use crate::walker::Walker;

pub(crate) struct FrameReadout {
    since: Instant,
    frames: u32,
}

impl FrameReadout {
    pub(crate) fn new(now: Instant) -> Self {
        Self {
            since: now,
            frames: 0,
        }
    }

    fn presented(&mut self, now: Instant) -> Option<f64> {
        self.frames += 1;
        let elapsed = now - self.since;
        if elapsed < Duration::from_millis(500) {
            return None;
        }
        let fps = f64::from(self.frames) / elapsed.as_secs_f64();
        self.since = now;
        self.frames = 0;
        Some(fps)
    }

    pub(crate) fn title(&mut self, now: Instant, walker: &Walker) -> Option<String> {
        let fps = self.presented(now)?;
        let coords = walker.coordinates();
        let [x, y, z] = coords.position;
        let [dx, dy, dz] = coords.direction;
        Some(format!(
            "Hypergame | {fps:.1} FPS | pos ({x:.3}, {y:.3}, {z:.3}) R | dir ({dx:.3}, {dy:.3}, {dz:.3})"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_counts_presented_frames_over_each_elapsed_interval() {
        let start = Instant::now();
        let mut readout = FrameReadout::new(start);
        for elapsed in [120, 180, 320] {
            assert_eq!(
                readout.presented(start + Duration::from_millis(elapsed)),
                None
            );
        }
        assert_eq!(
            readout.presented(start + Duration::from_millis(500)),
            Some(8.0)
        );
        // One slow frame occupies the entire next sample.
        assert_eq!(
            readout.presented(start + Duration::from_millis(2500)),
            Some(0.5)
        );
    }
}
