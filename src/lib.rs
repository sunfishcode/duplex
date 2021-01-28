//! The `Duplex` trait: interactive streams

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod duplex;

#[cfg(feature = "futures-io")]
pub use crate::duplex::FullDuplex;
#[cfg(feature = "tokio")]
pub use crate::duplex::TokioFullDuplex;
pub use crate::duplex::{Duplex, HalfDuplex};
