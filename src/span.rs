//! Source span type alias.

use std::ops::Range;

/// Byte range in the source text.
pub type Span = Range<usize>;
