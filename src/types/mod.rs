pub mod resolved_types;
pub mod spanned_types;
mod tokens;

use std::ops::Range;

pub use tokens::Token;
pub type Span = Range<usize>;
