use crate::error::WidgetBaseError;
use std::num::FpCategory;
use std::ops::Deref;

#[derive(Debug, thiserror::Error)]
pub enum MathError {
	#[error("Non-normal used in context where only normal and zero floats are acceptable")]
	AbnormalFloat(FpCategory),

	#[error("Negative float use in context where only positive floats are acceptable")]
	NegativeFloat(f32),
}

/// An f32 that is not:
/// - NaN
/// - INF / -INF
/// - Subnormal
/// Zero, including negative zero, is a valid state.
/// Use `ValidF32<true>` for restricting to positive states only.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ValidF32<const POSITIVE: bool = false>(f32);

impl ValidF32<false> {
	pub const NEGATIVE_ONE: Self = ValidF32(-1.);
}

impl<const POSITIVE: bool> ValidF32<POSITIVE> {
	pub const POSITIVE_ONE: Self = ValidF32(1.);
	pub const ZERO: Self = ValidF32(0.);

	pub const fn get(self: Self) -> f32 {
		self.0
	}

	/// # Errors
	/// If one of the following:
	/// - n is not [`FpCategory::Normal`] or [`FpCategory::Zero`]
	/// - Self has the POSITIVE set to `true` and `n` is negative
	pub fn new(n: f32) -> Result<Self, MathError> {
		match n.classify() {
			FpCategory::Normal | FpCategory::Zero if !POSITIVE || n.is_sign_positive() => Ok(Self(n)),
			FpCategory::Normal | FpCategory::Zero => Err(MathError::NegativeFloat(n)),
			class => Err(MathError::AbnormalFloat(class)),
		}
	}

	/// # Safety
	/// The [`FpCategory`] of `n` must be [`FpCategory::Normal`] or [`FpCategory::Zero`].
	/// For `ValidF32<true>`, `n` must also be positive.
	pub unsafe fn new_unchecked(n: f32) -> Self {
		Self(n)
	}
}

impl<const POSITIVE: bool> Deref for ValidF32<POSITIVE> {
	type Target = f32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const POSITIVE: bool> From<ValidF32<POSITIVE>> for f32 {
	fn from(ValidF32(value): ValidF32<POSITIVE>) -> Self {
		value
	}
}

impl<const POSITIVE: bool> TryFrom<f32> for ValidF32<POSITIVE> {
	type Error = MathError;

	fn try_from(value: f32) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

impl From<ValidF32<true>> for ValidF32<false> {
	fn from(ValidF32::<true>(f32): ValidF32<true>) -> Self {
		ValidF32::<false>(f32)
	}
}

impl TryFrom<ValidF32<false>> for ValidF32<true> {
	type Error = MathError;

	fn try_from(ValidF32::<false>(f32): ValidF32<false>) -> Result<Self, Self::Error> {
		if f32.is_sign_positive() {
			Ok(ValidF32::<true>(f32))
		} else {
			Err(MathError::NegativeFloat(f32))
		}
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ValidVec2<const POSITIVE: bool = false> {
	pub x: ValidF32<POSITIVE>,
	pub y: ValidF32<POSITIVE>,
}

impl<const POSITIVE: bool> ValidVec2<POSITIVE> {
	pub const ZERO: Self = Self {
		x: ValidF32::ZERO,
		y: ValidF32::ZERO,
	};

	pub const fn get(self) -> [f32; 2] {
		[self.x.get(), self.y.get()]
	}

	pub fn new(x: f32, y: f32) -> Result<Self, MathError> {
		Ok(Self {
			x: x.try_into()?,
			y: y.try_into()?,
		})
	}

	/// # Safety
	/// Both `x` and `y` must follow the constraints of a [`ValidF32`].
	/// See the safety section of [`ValidF32::new_unchecked`]
	pub unsafe fn new_unchecked(x: f32, y: f32) -> Self {
		Self {
			x: ValidF32::new_unchecked(x),
			y: ValidF32::new_unchecked(y),
		}
	}
}

impl<const POSITIVE: bool> From<ValidVec2<POSITIVE>> for [f32; 2] {
	fn from(value: ValidVec2<POSITIVE>) -> Self {
		value.get()
	}
}

impl<const POSITIVE: bool> TryFrom<[f32; 2]> for ValidVec2<POSITIVE> {
	type Error = MathError;

	fn try_from([x, y]: [f32; 2]) -> Result<Self, Self::Error> {
		Ok(Self {
			x: x.try_into()?,
			y: y.try_into()?,
		})
	}
}

//TODO: maybe base this off traits that yield an f32? or make a macro
impl From<[i8; 2]> for ValidVec2 {
	fn from([x, y]: [i8; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl From<[i16; 2]> for ValidVec2 {
	fn from([x, y]: [i16; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl From<[i32; 2]> for ValidVec2 {
	fn from([x, y]: [i32; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl From<[i64; 2]> for ValidVec2 {
	fn from([x, y]: [i64; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl From<[i128; 2]> for ValidVec2 {
	fn from([x, y]: [i128; 2]) -> Self {
		//i128::MAX -> f32 works fine
		//i128::MIN -> f32 works fine
		//u128::MAX -> f32 gives INF
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl TryFrom<[i8; 2]> for ValidVec2<true> {
	type Error = MathError;

	fn try_from([x, y]: [i8; 2]) -> Result<Self, Self::Error> {
		ValidVec2::new(x as f32, y as f32)
	}
}

impl TryFrom<[i16; 2]> for ValidVec2<true> {
	type Error = MathError;

	fn try_from([x, y]: [i16; 2]) -> Result<Self, Self::Error> {
		ValidVec2::new(x as f32, y as f32)
	}
}

impl TryFrom<[i32; 2]> for ValidVec2<true> {
	type Error = MathError;

	fn try_from([x, y]: [i32; 2]) -> Result<Self, Self::Error> {
		ValidVec2::new(x as f32, y as f32)
	}
}

impl TryFrom<[i64; 2]> for ValidVec2<true> {
	type Error = MathError;

	fn try_from([x, y]: [i64; 2]) -> Result<Self, Self::Error> {
		ValidVec2::new(x as f32, y as f32)
	}
}

impl TryFrom<[i128; 2]> for ValidVec2<true> {
	type Error = MathError;

	fn try_from([x, y]: [i128; 2]) -> Result<Self, Self::Error> {
		ValidVec2::new(x as f32, y as f32)
	}
}

impl<const POSITIVE: bool> From<[u8; 2]> for ValidVec2<POSITIVE> {
	fn from([x, y]: [u8; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl<const POSITIVE: bool> From<[u16; 2]> for ValidVec2<POSITIVE> {
	fn from([x, y]: [u16; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl<const POSITIVE: bool> From<[u32; 2]> for ValidVec2<POSITIVE> {
	fn from([x, y]: [u32; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl<const POSITIVE: bool> From<[u64; 2]> for ValidVec2<POSITIVE> {
	fn from([x, y]: [u64; 2]) -> Self {
		unsafe { Self::new_unchecked(x as f32, y as f32) }
	}
}

impl<const POSITIVE: bool> TryFrom<[u128; 2]> for ValidVec2<POSITIVE> {
	type Error = MathError;

	fn try_from([x, y]: [u128; 2]) -> Result<Self, Self::Error> {
		//i128::MAX -> f32 works fine
		//i128::MIN -> f32 works fine
		//u128::MAX -> f32 gives INF, so we must treat it as normal
		ValidVec2::new(x as f32, y as f32)
	}
}
