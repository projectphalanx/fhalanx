pub use brush::contracts::traits::psp22::*;

#[brush::wrapper]
pub type Psp22TokenRef = dyn PSP22;

#[brush::trait_definition]
pub trait Psp22Token: PSP22 {}
