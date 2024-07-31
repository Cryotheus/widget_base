use crate::error::WidgetBaseError;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Align {
	/// Align to the starting, top, or left side.
	Negative,

	/// Align to the center.
	Zero,

	/// Align to the ending, bottom, or right side.
	Positive,
}

impl Align {
	pub const fn scalar(self) -> f32 {
		match self {
			Self::Negative => -1.,
			Self::Zero => 0.,
			Self::Positive => 1.,
		}
	}
}

impl From<Align> for f32 {
	fn from(value: Align) -> Self {
		value.scalar()
	}
}

impl TryFrom<f32> for Align {
	type Error = WidgetBaseError;

	fn try_from(value: f32) -> Result<Self, Self::Error> {
		match WidgetBaseError::validate_f32(value)?.signum() {
			-1. => todo!(),
			0. => todo!(),
			1. => todo!(),
			_ => unreachable!(),
		}
	}
}
