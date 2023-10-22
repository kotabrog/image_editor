mod renderer;
mod image;
mod input;
mod image_data;
mod button;
mod anchor;
mod canvas;
mod rect;
mod label;

pub use renderer::Renderer;
pub use image::Image;
pub use input::Input;
pub use image_data::ImageDataWrapper;
pub use button::Button;
pub use anchor::Anchor;
pub use canvas::Canvas;
pub use rect::Rect;
pub use label::Label;

pub trait DisplayElement: std::fmt::Debug {
    fn set_disabled(&self, disabled: bool);
}
