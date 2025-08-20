#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod backend;
mod chart;
pub mod charts;

pub use backend::{EguiBackend, EguiBackendError};
pub use chart::{
    Chart, MouseButton, MouseConfig, Transform, DEFAULT_MOVE_SCALE, DEFAULT_SCROLL_SCALE,
};

#[cfg(feature = "timechart")]
use std::ops::Range;

#[cfg(feature = "timechart")]
fn mult_range(range: Range<f32>, mult: f32) -> Range<f32> {
    let delta = range.end - range.start;

    let half_delta = delta / 2.0;

    let midpoint = range.end - half_delta;

    let adjusted_delta = half_delta * mult;

    let start = midpoint - adjusted_delta;
    let end = midpoint + adjusted_delta;

    Range { start, end }
}
