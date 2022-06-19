/// Vec3 is a struct representing a three dimensional vector
#[derive(Debug)]
pub struct Vec3 {
    coordinates: [f64; 3],
}

impl Vec3 {
    // creates a new Vec3 with coordinates set appropriately
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            coordinates: [x, y, z],
        }
    }

    pub fn x(&self) -> f64 {
        self.coordinates[0]
    }

    pub fn y(&self) -> f64 {
        self.coordinates[1]
    }

    pub fn z(&self) -> f64 {
        self.coordinates[2]
    }

    pub fn sub(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            coordinates: [self.x() - v.x(), self.y() - v.y(), self.z() - v.z()],
        }
    }

    pub fn add(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            coordinates: [self.x() + v.x(), self.y() + v.y(), self.z() + v.z()],
        }
    }

    pub fn multiply(&self, n: f64) -> Vec3 {
        Vec3 {
            coordinates: [self.x() * n, self.y() * n, self.z() * n],
        }
    }

    pub fn div(&self, n: f64) -> Vec3 {
        Vec3 {
            coordinates: [self.x() / n, self.y() / n, self.z() / n],
        }
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn into_unit_vec(&self) -> Vec3 {
        self.div(self.len())
    }

    fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    fn len(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}\n", self.x(), self.y(), self.z())
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn print_color(color: &Color) {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;
    print!("{} {} {}\n", ir, ig, ib)
}
