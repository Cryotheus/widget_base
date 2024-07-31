use crate::behavior::Behavior;
use crate::error::WidgetBaseError;
use crate::layout::Layout;
use crate::math::ValidVec2;
use crate::widget::id::WidgetId;
use crate::widget::widget_base::BuildWidget;
use crate::widget::Widget;
use std::fmt::Debug;

#[derive(Debug)]
pub struct WidgetBuilder {
	pub behavior: Option<Box<dyn Behavior>>,
	pub layout: Option<Box<dyn Layout>>,

	pub parent_id: Option<WidgetId>,

	pub size: ValidVec2<true>,
	pub position: ValidVec2,
}

impl WidgetBuilder {
	//TODO: remove and set fns for stuff other than parent_id
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_behavior(mut self, behavior: impl Into<Box<dyn Behavior>>) -> Self {
		self.behavior = Some(behavior.into());

		self
	}

	pub fn with_layout(mut self, layout: impl Into<Box<dyn Layout>>) -> Self {
		self.layout = Some(layout.into());

		self
	}

	pub fn with_parent_id(mut self, parent: WidgetId) -> Self {
		self.parent_id = Some(parent);

		self
	}

	/// # Panics
	/// If `size` cannot be converted into a [`ValidVec2<false>`].
	pub fn with_position<V>(mut self, size: V) -> Self
	where
		V: TryInto<ValidVec2<false>>,
		<V as TryInto<ValidVec2<false>>>::Error: Debug,
	{
		self.position = size.try_into().unwrap();

		self
	}

	/// # Panics
	/// If `size` cannot be converted into a [`ValidVec2<true>`].
	pub fn with_size<P>(mut self, size: P) -> Self
	where
		P: TryInto<ValidVec2<true>>,
		<P as TryInto<ValidVec2<true>>>::Error: Debug,
	{
		//TODO: unwrap -> expect
		self.size = size.try_into().unwrap();

		self
	}

	pub fn without_behavior(mut self) -> Self {
		self.behavior = None;

		self
	}

	pub fn without_layout(mut self) -> Self {
		self.layout = None;

		self
	}

	pub fn without_parent_id(mut self) -> Self {
		self.parent_id = None;

		self
	}

	pub fn remove_parent_id(&mut self) {
		self.parent_id = None;
	}

	pub fn set_parent_id(&mut self, parent_id: WidgetId) {
		self.parent_id = Some(parent_id);
	}

	pub fn try_clone(&self) -> Result<Self, WidgetBaseError> {
		match self {
			#[allow(unused_qualifications)]
			WidgetBuilder {
				//must be prefixed by Option or postfixed with a type
				//otherwise rust's lexer sees it as a variable binding and not a pattern
				behavior: None::<_>,
				layout: None::<_>,
				size,
				position,
				parent_id: parent,
			} => Ok(WidgetBuilder {
				behavior: None,
				layout: None,

				parent_id: *parent,
				size: *size,
				position: *position,
			}),

			_ => Err(WidgetBaseError::TraitObjectCloning),
		}
	}
}

impl BuildWidget for WidgetBuilder {
	fn build_widget(self, id: WidgetId) -> Widget {
		Widget {
			behavior: self.behavior,
			child_indices: Vec::new(),
			id,
			layout: self.layout,
			parent_id: None,
			position: Default::default(),
			size: self.size,
		}
	}
}

impl Default for WidgetBuilder {
	fn default() -> Self {
		Self {
			behavior: None,
			layout: None,
			parent_id: None,
			size: ValidVec2::ZERO,
			position: ValidVec2::ZERO,
		}
	}
}

//cursed...
impl TryFrom<&WidgetBuilder> for WidgetBuilder {
	type Error = WidgetBaseError;

	fn try_from(builder_ref: &WidgetBuilder) -> Result<Self, Self::Error> {
		builder_ref.try_clone()
	}
}
