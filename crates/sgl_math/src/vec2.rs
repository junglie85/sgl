use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use mint::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(&self) -> Vec2 {
        let len = self.len();

        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn perp_ccw(&self) -> Vec2 {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn perp_cw(&self) -> Vec2 {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn dot(&self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
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

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
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

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
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

impl From<(u32, u32)> for Vec2 {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_length_of_vec2() {
        let v = v2(3.0, 4.0);

        assert_eq!(5.0, v.len());
    }

    #[test]
    fn calculate_normal_of_vec2() {
        let v = v2(3.0, 4.0);

        assert_eq!(v2(3.0, 4.0) / 5.0, v.norm());
    }

    #[test]
    fn calculate_counter_clockwise_perpendicular_of_vec2() {
        let v = v2(10.0, 5.0);

        assert_eq!(v2(-5.0, 10.0), v.perp_ccw())
    }

    #[test]
    fn calculate_clockwise_perpendicular_of_vec2() {
        let v = v2(10.0, 5.0);

        assert_eq!(v2(5.0, -10.0), v.perp_cw())
    }

    #[test]
    fn calculate_dot_product_of_two_vec2() {
        let a = v2(2.0, 4.0);
        let b = v2(4.0, 8.0);

        assert_eq!(40.0, a.dot(b))
    }

    #[test]
    fn vec2_to_array() {
        let v = v2(10.0, 5.0);

        assert_eq!([10.0, 5.0], v.to_array())
    }

    #[test]
    fn negate_vec2() {
        let a = v2(2.0, 4.0);
        let b = -a;

        assert_eq!(b, v2(-2.0, -4.0));
        assert_eq!(-b, a);
    }

    #[test]
    fn vec2_add_vec2() {
        let a = v2(2.0, 4.0);
        let b = v2(1.0, 1.0);

        assert_eq!(a + b, v2(3.0, 5.0));
        assert_eq!(b + a, v2(3.0, 5.0));
    }

    #[test]
    fn vec2_add_assign_vec2() {
        let mut a = v2(2.0, 4.0);
        a += v2(1.0, 1.0);

        assert_eq!(a, v2(3.0, 5.0));
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
    fn vec2_mul_vec2() {
        let a = v2(2.0, 4.0);
        let b = v2(3.0, 5.0);

        assert_eq!(a * b, v2(6.0, 20.0));
    }

    #[test]
    fn vec2_mul_assign_vec2() {
        let mut a = v2(2.0, 4.0);
        a *= v2(3.0, 5.0);

        assert_eq!(a, v2(6.0, 20.0));
    }

    #[test]
    fn vec2_mul_scalar() {
        let a = v2(2.0, 4.0);

        assert_eq!(a * 2.0, v2(4.0, 8.0));
    }

    #[test]
    fn vec2_mul_assign_scalar() {
        let mut a = v2(2.0, 4.0);
        a *= 2.0;

        assert_eq!(a, v2(4.0, 8.0));
    }

    #[test]
    fn vec2_div_scalar() {
        let a = v2(2.0, 4.0);

        assert_eq!(a / 2.0, v2(1.0, 2.0));
    }

    #[test]
    fn vec2_div_assign_scalar() {
        let mut a = v2(2.0, 4.0);
        a /= 2.0;

        assert_eq!(a, v2(1.0, 2.0));
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

    #[test]
    fn u32_tuple_into_vec2() {
        let v = (1_u32, 2_u32).into();

        assert_eq!(v2(1.0, 2.0), v);
    }

    #[test]
    fn f32_array_into_vec2() {
        let a = [3.0_f32, 4.0_f32];
        let v: Vec2 = a.into();

        assert_eq!(a[0], v.x);
        assert_eq!(a[1], v.y);
    }
}
