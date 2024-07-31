use std::fmt::{Debug, Formatter};
use crate::layout::Layout;

/// Interactive capabilities for a widget.
/// e.g.
/// - When the widget is clicked, or typed into
/// - When a frame renders
/// - Etc. etc.
pub trait Behavior {
	/// Optional implementation of [core::fmt::Debug::fmt].
	fn debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

impl Debug for dyn Behavior {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.debug(f)
	}
}