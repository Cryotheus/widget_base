use std::fmt::{Debug, Formatter};

/// Controller for the size and positioning of child widgets.
pub trait Layout {
	/// Optional implementation of [core::fmt::Debug::fmt].
	fn debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

impl Debug for dyn Layout {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.debug(f)
	}
}