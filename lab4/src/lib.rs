use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    pub const fn cross_abs(self, other: Self) -> f32 {
        self.x * other.y - other.x * self.y
    }
}

impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RotationDirection {
    Right,
    Left,
    NoRotation,
}

fn evaluate_rotation_direction(v1: Vec2d, v2: Vec2d) -> RotationDirection {
    let cross_abs = v1.cross_abs(v2);
    if cross_abs < -f32::EPSILON {
        RotationDirection::Right
    } else if cross_abs > f32::EPSILON {
        RotationDirection::Left
    } else {
        RotationDirection::NoRotation
    }
}

pub fn evaluate_rotations(polyline: &[Vec2d]) -> Box<[RotationDirection]> {
    const MIN_SIZE: usize = 3;

    if polyline.len() < MIN_SIZE {
        Box::new([])
    } else {
        polyline
            .windows(MIN_SIZE)
            .map(|window| (window[0], window[1], window[2]))
            .map(|(p1, p2, p3)| (p2 - p1, p3 - p2))
            .map(|(v1, v2)| evaluate_rotation_direction(v1, v2))
            .collect::<Box<[RotationDirection]>>()
    }
}
