use std::ops::{Neg, Sub, SubAssign};

use mint::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn v2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<Vec2> for Vector2<f32> {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vector2<f32>> for Vec2 {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negate_vec2() {
        let a = v2(2.0, 4.0);
        let b = -a;

        assert_eq!(b, v2(-2.0, -4.0));
        assert_eq!(-b, a);
    }

    #[test]
    fn vec2_subtract_vec2() {
        let a = v2(2.0, 4.0);
        let b = v2(1.0, 1.0);

        assert_eq!(a - b, v2(1.0, 3.0));
        assert_eq!(b - a, v2(-1.0, -3.0));
    }

    #[test]
    fn vec2_subtract_assign_vec2() {
        let mut a = v2(2.0, 4.0);
        a -= v2(1.0, 1.0);

        assert_eq!(a, v2(1.0, 3.0));
    }

    #[test]
    fn vec2_into_mint_vector2() {
        let v = v2(1.0, 2.0);
        let m: mint::Vector2<f32> = v.into();

        assert_eq!(v.x, m.x);
        assert_eq!(v.y, m.y);
    }

    #[test]
    fn mint_vector2_into_vec2() {
        let m = mint::Vector2::<f32> { x: 3.0, y: 4.0 };
        let v: Vec2 = m.into();

        assert_eq!(m.x, v.x);
        assert_eq!(m.y, v.y);
    }
}
