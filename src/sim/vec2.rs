use std::ops;
use std::fmt;

///A 2D Vector that can be
///  - added by another Vec2
///  - subtracted by another Vec2
///  - multiplied by a scalar value (`f64`)
///  - normalized
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn normalize(&self) -> Vec2 {
        let len_squared = self.x * self.x + self.y * self.y;
        let inv_sqrt = len_squared.sqrt();

        Vec2 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn abs(self) -> Vec2 {
        Vec2 {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

    pub fn from_arr(v: [f64; 2]) -> Vec2 {
        Vec2 {
            x: v[0],
            y: v[1]
        }
    }

    pub fn dist_scalar(a: Vec2, b: Vec2) -> f64 {
        let absq = (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y);
        absq.sqrt()
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// `vec2!` macro
///  ## Purpose
///  shorthand for writing `Vec2 { x, y }`
///  ## Usage
///  ```
///  let a = Vec2 {x: 10, y: 20};
///  let b = vec2!(10, 20);
///
///  assert_eq!(a, b); // true
///  ```
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2 {
            x: $x as f64,
            y: $y as f64
        }
    };
    ($v:expr) => {
        vec2!($v, $v)
    };
    () => {
        vec2!(0)
    }
}
