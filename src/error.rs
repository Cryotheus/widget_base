use std::num::FpCategory;

#[derive(Debug, thiserror::Error)]
pub enum WidgetBaseError {
	#[error("A value of 0 cannot be used for an instance of NonZeroUsize")]
	InvalidNonZeroUsize,
	
	#[error("Non-normal float provided in context where only normal floats are acceptable")]
	AbnormalFloat(FpCategory),
	
	#[error("Attempted to clone a datatype containing instances of trait objects")]
	TraitObjectCloning,
}

impl WidgetBaseError {
	pub fn validate_f32(f32: f32) -> Result<f32, WidgetBaseError> {
		match f32.classify() {
			FpCategory::Zero | FpCategory::Normal => Ok(f32),
			abnormal_class => Err(WidgetBaseError::AbnormalFloat(abnormal_class)),
		}
	}
}