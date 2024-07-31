pub mod builder;
pub mod id;
pub mod widget_base;

use crate::behavior::Behavior;
use crate::layout::Layout;
use crate::math::ValidVec2;
use id::WidgetId;

#[derive(Debug)]
pub struct Widget {
	pub(crate) id: WidgetId,
	pub(crate) child_indices: Vec<usize>,
	pub(crate) parent_id: Option<WidgetId>,

	pub(crate) behavior: Option<Box<dyn Behavior>>,
	pub(crate) layout: Option<Box<dyn Layout>>,

	pub(crate) size: ValidVec2<true>,
	pub(crate) position: ValidVec2,
}

impl Widget {
	pub fn id(&self) -> WidgetId {
		self.id
	}

	pub(crate) fn index(&self) -> usize {
		self.id.to_index()
	}

	pub fn parent_id(&self) -> Option<WidgetId> {
		self.parent_id
	}

	pub fn position(&self) -> ValidVec2 {
		self.position
	}

	pub fn size(&self) -> ValidVec2<true> {
		self.size
	}

	pub fn take_behavior(&mut self) -> Option<Box<dyn Behavior>> {
		self.behavior.take()
	}

	pub fn take_layout(&mut self) -> Option<Box<dyn Layout>> {
		self.layout.take()
	}
}
