mod any;
mod cop;

pub use any::*;
pub type Result<T> = std::result::Result<T, std::convert::Infallible>;

pub fn ok<Target, Index, S: cop::Init<Target, Index>>(target: Target) -> Result<S> {
    Ok(S::init(target))
}
