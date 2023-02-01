use crate::Vec2;

pub struct Aabb {
    min: Vec2,
    max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }
}

#[cfg(test)]
mod tests {
    use crate::v2;

    use super::*;

    #[test]
    fn calculate_aabb_width() {
        let aabb = Aabb::new(v2(10.0, 10.0), v2(20.0, 30.0));

        assert_eq!(10.0, aabb.width());
    }

    #[test]
    fn calculate_aabb_height() {
        let aabb = Aabb::new(v2(10.0, 10.0), v2(20.0, 30.0));

        assert_eq!(20.0, aabb.height());
    }

    #[test]
    fn calculate_aabb_center() {
        let aabb = Aabb::new(v2(10.0, 10.0), v2(20.0, 30.0));

        assert_eq!(v2(15.0, 20.0), aabb.center());
    }
}
