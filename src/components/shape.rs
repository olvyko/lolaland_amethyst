use amethyst::ecs::prelude::{Component, VecStorage};

///Component represents the shape of an entity
#[derive(Debug, Clone)]
pub enum Shape {
  Rect { width: f64, height: f64 },
}

impl Component for Shape {
  type Storage = VecStorage<Self>;
}
