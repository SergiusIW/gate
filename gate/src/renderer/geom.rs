// Copyright 2017-2020 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ops::{Add, Mul};

// 2D Cartesian vector
#[derive(Copy, Clone)]
pub struct Vec2 { pub x: f64, pub y: f64 }

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 { Vec2 { x, y } }
    pub fn zero() -> Vec2 { Vec2::new(0.0, 0.0) }
    pub fn len(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x + rhs.x, self.y + rhs.y) }
}

// Diagonal 2D matrix
#[derive(Copy, Clone)]
struct Diag2 { x: f64, y: f64 }

impl Diag2 {
    fn new(x: f64, y: f64) -> Diag2 { Diag2 { x, y } }
}

impl Mul<Vec2> for Diag2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x*rhs.x, self.y*rhs.y) }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 { Vec2::new(self * rhs.x, self * rhs.y) }
}

// General 2D matrix
#[derive(Copy, Clone)]
pub struct Mat2 { a: f64, b: f64, c: f64, d: f64 }

impl Mat2 {
    fn id() -> Mat2 { Mat2 { a: 1.0, b: 0.0, c: 0.0, d: 1.0 } }

    fn rotation(angle: f64) -> Mat2 {
        let (sin, cos) = (angle.sin(), angle.cos());
        let (sin, cos) = if sin.abs() < 1e-7 || cos.abs() < 1e-7 {
            (sin.round(), cos.round())
        } else {
            (sin, cos)
        };
        Mat2 { a: cos, b: -sin, c: sin, d: cos }
    }

    pub fn col_0(&self) -> Vec2 { Vec2::new(self.a, self.c) }
    pub fn col_1(&self) -> Vec2 { Vec2::new(self.b, self.d) }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.a * rhs.x + self.b * rhs.y, self.c * rhs.x + self.d * rhs.y)
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2 {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}

impl Mul<Diag2> for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Diag2) -> Mat2 {
        Mat2 { a: self.a * rhs.x, b: self.b * rhs.y, c: self.c * rhs.x, d: self.d * rhs.y }
    }
}

impl Mul<Mat2> for Diag2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2 { a: self.x * rhs.a, b: self.x * rhs.b, c: self.y * rhs.c, d: self.y * rhs.d }
    }
}

impl Mul<Mat2> for f64 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2 { a: self * rhs.a, b: self * rhs.b, c: self * rhs.c, d: self * rhs.d }
    }
}

/// Represents an affine transformation in 2D space.
#[derive(Copy, Clone)]
pub struct Affine { mat: Mat2, offset: Vec2 }

impl Affine {
    /// Identity transformation.
    #[inline]
    pub fn id() -> Affine { Affine { mat: Mat2::id(), offset: Vec2::zero() } }

    /// Returns a translation transformation.
    pub fn translate(x_offset: f64, y_offset: f64) -> Affine {
        Affine { mat: Mat2::id(), offset: Vec2::new(x_offset, y_offset) }
    }

    /// Returns a rotation transformation, rotating counter-clockwise by `angle` radians.
    pub fn rotate(angle: f64) -> Affine {
        Affine { mat: Mat2::rotation(angle), offset: Vec2::zero() }
    }

    /// Returns a scaling transformation, scaling x and y axes separately.
    pub fn scale_axes(scale_x: f64, scale_y: f64) -> Affine {
        Affine { mat: Mat2 { a: scale_x, b: 0., c: 0., d: scale_y }, offset: Vec2::zero() }
    }

    /// Returns a scaling transformation, scaling x and y axes identically.
    pub fn scale(scale: f64) -> Affine {
        Affine::scale_axes(scale, scale)
    }

    /// Returns a transformation that is functionally equivalent to `self` composed with `rhs`.
    ///
    /// This means that the `rhs` transformation is invoked first, and then the `self`
    /// transformation is invoked on the output of that.
    /// This is usually the desired ordering with graphics transformations.
    pub fn pre_transform(&self, rhs: &Affine) -> Affine {
        Affine {
            mat: self.mat * rhs.mat,
            offset: self.offset + self.mat * rhs.offset,
        }
    }

    /// Logically equivalent to `self.pre_transform(&Affine::scale_axes(scale_x, scale_y))`.
    pub fn pre_scale_axes(&self, scale_x: f64, scale_y: f64) -> Affine {
        Affine {
            mat: self.mat * Diag2::new(scale_x, scale_y),
            offset: self.offset,
        }
    }

    /// Logically equivalent to `self.pre_transform(&Affine::scale(scale))`.
    pub fn pre_scale(&self, scale: f64) -> Affine {
        Affine {
            mat: scale * self.mat,
            offset: self.offset,
        }
    }

    /// Logically equivalent to `self.pre_transform(&Affine::rotate(angle))`.
    pub fn pre_rotate(&self, angle: f64) -> Affine {
        Affine {
            mat: self.mat * Mat2::rotation(angle),
            offset: self.offset,
        }
    }

    /// Logically equivalent to `self.pre_transform(&Affine::translate(x_offset, y_offset))`.
    pub fn pre_translate(&self, x_offset: f64, y_offset: f64) -> Affine {
        Affine {
            mat: self.mat,
            offset: self.offset + self.mat * Vec2::new(x_offset, y_offset),
        }
    }

    /// Logically equivalent to `Affine::scale_axes(scale_x, scale_y).pre_transform(self)`.
    pub fn post_scale_axes(&self, scale_x: f64, scale_y: f64) -> Affine {
        let scale = Diag2::new(scale_x, scale_y);
        Affine {
            mat: scale * self.mat,
            offset: scale * self.offset,
        }
    }

    /// Logically equivalent to `Affine::scale(scale).pre_transform(self)`.
    pub fn post_scale(&self, scale: f64) -> Affine {
        Affine {
            mat: scale * self.mat,
            offset: scale * self.offset,
        }
    }

    /// Logically equivalent to `Affine::rotate(angle).pre_transform(self)`.
    pub fn post_rotate(&self, angle: f64) -> Affine {
        let rotation = Mat2::rotation(angle);
        Affine {
            mat: rotation * self.mat,
            offset: rotation * self.offset,
        }
    }

    /// Logically equivalent to `Affine::translate(x_offset, y_offset).pre_transform(self)`.
    pub fn post_translate(&self, x_offset: f64, y_offset: f64) -> Affine {
        Affine {
            mat: self.mat,
            offset: self.offset + Vec2::new(x_offset, y_offset),
        }
    }

    pub(crate) fn apply(&self, input: Vec2) -> Vec2 { self.mat * input + self.offset }

    pub(crate) fn apply_f32(&self, input: (f32, f32)) -> (f32, f32) {
        let input = Vec2::new(input.0 as f64, input.1 as f64);
        let result = self.apply(input);
        (result.x as f32, result.y as f32)
    }

    pub(crate) fn mat(&self) -> &Mat2 { &self.mat }
}
