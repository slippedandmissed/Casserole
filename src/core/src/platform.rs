use crate::graphics::GraphicsLibrary;
use crate::logging::Logger;

pub trait Platform {
    fn graphics(&self) -> &dyn GraphicsLibrary;
    fn logger(&self) -> &dyn Logger;
}