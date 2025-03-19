// Physics and collision systems
mod collision;
mod platforms;

pub use collision::{CollisionType, Platform, World};
pub use platforms::PlatformManager;