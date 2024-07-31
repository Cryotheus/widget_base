use std::num::{NonZero, NonZeroUsize};
use std::ops::Deref;
use crate::error::WidgetBaseError;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WidgetId(NonZeroUsize);

impl WidgetId {
	pub const fn from_index(index: usize) -> Self {
		match NonZeroUsize::new(index + 1) {
			None => panic!("WidgetId::from_index used a zero usize"),
			Some(id) => WidgetId(id),
		}
	}

	pub const fn new(id: NonZeroUsize) -> Self {
		WidgetId(id)
	}

	pub const fn to_index(self) -> usize {
		(self.0.get()) - 1
	}

	pub const fn to_usize(self) -> usize {
		self.0.get()
	}
}

impl<U> AsRef<U> for WidgetId
where
	<WidgetId as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl Deref for WidgetId {
	type Target = NonZeroUsize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<NonZeroUsize> for WidgetId {
	fn from(value: NonZeroUsize) -> Self {
		WidgetId(value)
	}
}

impl TryFrom<usize> for WidgetId {
	type Error = WidgetBaseError;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match NonZeroUsize::new(value) {
			Some(id) => Ok(WidgetId(id)),
			None => Err(WidgetBaseError::InvalidNonZeroUsize),
		}
	}
}

impl TryFrom<Option<NonZeroUsize>> for WidgetId {
	type Error = WidgetBaseError;

	fn try_from(value: Option<NonZeroUsize>) -> Result<Self, Self::Error> {
		match value {
			Some(id) => Ok(WidgetId(id)),
			None => Err(WidgetBaseError::InvalidNonZeroUsize),
		}
	}
}

impl From<WidgetId> for NonZeroUsize {
	fn from(value: WidgetId) -> Self {
		value.0
	}
}

impl From<WidgetId> for usize {
	fn from(value: WidgetId) -> Self {
		value.0.get()
	}
}
