use std::ops::{ Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3]
    // instantiate size 3 float vector for coords
}
pub type Point3 = Vec3;
pub type Color = Vec3;
// creates aliases for vec3 - constrains it?

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
        // create new Vec3 obj and return
    }
}

//index function implementation for vec3 that works with usize index variable
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3; // in the native code https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[0] + other[0],
                self[1] + other[1],
                self[2] + other[2]
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [
                self[0] + other[0],
                self[1] + other[1],
                self[2] + other[2]
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3; // in the native code https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[0] - other[0],
                self[1] - other[1],
                self[2] - other[2]
            ]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [
                self[0] - other[0],
                self[1] - other[1],
                self[2] - other[2]
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3; // in the native code https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [
                self[0] * other,
                self[1] * other,
                self[2] * other
            ]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [
                self[0] * other,
                self[1] * other,
                self[2] * other
            ]
        }
    }
}

impl Mul<Vec3> for f64 {
    // allows multiplication of scalar to vector
    type Output = Vec3; // in the native code https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self * other[0],
                self * other[1],
                self * other[2]
            ]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3; // in the native code https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [
                self[0] / other,
                self[1] / other,
                self[2] / other
            ]
        }
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [
                self[0] - other,
                self[1] - other,
                self[2] - other
            ]
        }
    }
}

impl Vec3 {
    pub fn x(self) -> f64 {
        self[0]
    }
    
    pub fn y(self) -> f64 {
        self[1]
    }
    
    pub fn z(self) -> f64 {
        self[2]
    }
    
    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
        // because the vector length is squared components
    }
    
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]
            ]
        }
    }
    
    pub fn normalized(self) -> Vec3 {
        self / self.length()
        // convert to unit vector
    }
    
    pub fn format_color(self) -> String {
        format!("{} {} {}", (255.999 * self[0]) as u64,
                            (255.999 * self[1]) as u64,
                            (255.999 * self[2]) as u64)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}