//! Provides an animated color value, so you can have colors change over time

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use crate::{film::Colorf, linalg};

/// `ColorKeyframe` is a color associated with a specific time
#[derive(Debug, Copy, Clone)]
pub struct ColorKeyframe {
    pub color: Colorf,
    pub time: f32,
}

impl ColorKeyframe {
    pub fn new(color: &Colorf, time: f32) -> Self {
        Self {
            color: *color,
            time,
        }
    }
}
impl Ord for ColorKeyframe {
    fn cmp(&self, other: &ColorKeyframe) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for ColorKeyframe {
    fn partial_cmp(&self, other: &ColorKeyframe) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}
impl Eq for ColorKeyframe {}
impl PartialEq for ColorKeyframe {
    fn eq(&self, other: &ColorKeyframe) -> bool {
        self.time == other.time
    }
}

/// `AnimatedColor` is a list of colors associated with time points in the scene
/// that will compute the color at the desired time by blending the two nearest ones
#[derive(Debug, Clone)]
pub struct AnimatedColor {
    /// List of color keyframes in time order
    keyframes: Vec<ColorKeyframe>,
}

impl AnimatedColor {
    /// Create an animated transform that will blend between the passed keyframes
    pub fn with_keyframes(mut keyframes: Vec<ColorKeyframe>) -> Self {
        keyframes.sort();
        Self { keyframes }
    }
    /// Compute the color at the desired time
    pub fn color(&self, time: f32) -> Colorf {
        if self.keyframes.is_empty() {
            Colorf::black()
        } else if self.keyframes.len() == 1 {
            self.keyframes[0].color
        } else {
            // TODO: Binary search here?
            let first = self.keyframes.iter().take_while(|k| k.time < time).last();
            let second = self.keyframes.iter().find(|k| k.time >= time);

            match (first, second) {
                (None, _) => self.keyframes.first().unwrap().color,
                (_, None) => self.keyframes.last().unwrap().color,
                (Some(fk), Some(sk)) => {
                    let mut color = Colorf::black();
                    let t = (time - fk.time) / (sk.time - fk.time);
                    color.r = linalg::lerp(t, &fk.color.r, &sk.color.r);
                    color.g = linalg::lerp(t, &fk.color.g, &sk.color.g);
                    color.b = linalg::lerp(t, &fk.color.b, &sk.color.b);
                    color.a = linalg::lerp(t, &fk.color.a, &sk.color.a);
                    color
                }
            }
        }
    }
}
