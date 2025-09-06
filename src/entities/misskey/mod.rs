pub mod latest;
#[path="2025.7.0/mod.rs"]
pub mod v2025_7_0;

#[path="2025.6.1/mod.rs"]
pub mod v2025_6_1;

#[path="2025.6.0/mod.rs"]
pub mod v2025_6_0;

#[path="2025.5.1/mod.rs"]
pub mod v2025_5_1;

pub mod prelude {
    pub use super::latest::*;
    pub use super::v2025_7_0::*;
    pub use super::v2025_6_1::*;
    pub use super::v2025_6_0::*;
    pub use super::v2025_5_1::*;
}
