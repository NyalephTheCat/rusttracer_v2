use crate::{objects::Intersectable, utils::math::point::Point};

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox { min, max }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.max.x >= other.min.x
            && self.min.x <= other.max.x
            && self.max.y >= other.min.y
            && self.min.y <= other.max.y
            && self.max.z >= other.min.z
            && self.min.z <= other.max.z
    }

    pub fn intersects_ray(&self, origin: &Point, direction: &Point) -> bool {
        let mut tmin = (self.min.x - origin.x) / direction.x;
        let mut tmax = (self.max.x - origin.x) / direction.x;

        let tymin = (self.min.y - origin.y) / direction.y;
        let tymax = (self.max.y - origin.y) / direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.min.z - origin.z) / direction.z;
        let tzmax = (self.max.z - origin.z) / direction.z;

        !((tmin > tzmax) || (tzmin > tmax))
    }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn union_point(&self, point: &Point) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                self.min.x.min(point.x),
                self.min.y.min(point.y),
                self.min.z.min(point.z),
            ),
            max: Point::new(
                self.max.x.max(point.x),
                self.max.y.max(point.y),
                self.max.z.max(point.z),
            ),
        }
    }

    pub fn center(&self) -> Point {
        Point::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn size(&self) -> Point {
        Point::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    pub fn from_objects(objects: &[crate::objects::Object]) -> BoundingBox {
        let mut bbox = BoundingBox::empty();

        for object in objects {
            bbox = bbox.union(&(*object).bounding_box());
        }

        bbox
    }
}
