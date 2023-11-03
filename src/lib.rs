use nalgebra::{Matrix4, Vector2};
use nalgebra::Vector3;

pub mod io;
pub mod render;
pub mod engine;
pub mod app;
pub mod ui;

pub type Vec2i = Vector2<i32>;
pub type Vec2s = Vector2<usize>;
pub type Vec2 = Vector2<f32>;

pub type Vec3i = Vector3<i32>;
pub type Vec3 = Vector3<f32>;

pub type Mat4 = Matrix4<f32>;
