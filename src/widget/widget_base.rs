use crate::widget::id::WidgetId;
use crate::widget::Widget;
use std::collections::BTreeSet;
use std::mem;

/// Assigns numerical IDs to widgets,
/// and enforces hierarchical logic for widgets.
#[derive(Debug)]
pub struct WidgetBase {
	priority_ids: BTreeSet<WidgetId>,
	widgets: Vec<Option<Widget>>,
}

impl WidgetBase {
	pub fn insert(&mut self, widget: impl BuildWidget) -> WidgetId {
		let id = self.next_id();
		let index = id.to_index();
		let mut widget = widget.build_widget(id);

		//attempt to parent the widget, if we can
		if let Some(parent_id) = widget.parent_id {
			if let Some(Some(parent_widget)) = self.widgets.get_mut(index) {
				parent_widget.child_indices.push(parent_id.to_index());
			} else {
				widget.parent_id = None;
			}
		}

		if self.widgets.len() == index {
			self.widgets.push(Some(widget));
		} else {
			self.widgets[id.to_index()] = Some(widget);
		}

		id
	}

	pub fn new() -> Self {
		Self {
			priority_ids: BTreeSet::new(),
			widgets: Vec::new(),
		}
	}

	fn next_id(&mut self) -> WidgetId {
		match self.priority_ids.pop_first() {
			None => WidgetId::from_index(self.widgets.len()),
			Some(id) => id,
		}
	}

	pub fn remove(&mut self, id: WidgetId) -> Option<Widget> {
		let index = id.to_index();
		let widget_count = self.widgets.len();

		if widget_count == 0 {
			return None;
		}

		let max_widget_index = widget_count - 1;

		if index == max_widget_index {
			//we're removing the last item, do special stuff
			let widget = self.widgets.pop().unwrap().unwrap();
			let mut lowest_pop_index: Option<usize> = None;

			//remove None entries as they were just holding up the entry we removed
			loop {
				let last = self.widgets.last();

				if let Some(last) = last {
					if last.is_none() {
						self.widgets.pop();
						lowest_pop_index = Some(lowest_pop_index.unwrap_or(usize::MAX).min(self.widgets.len()));

						continue;
					}
				}

				break;
			}

			//update priority_ids
			if let Some(lowest_pop_index) = lowest_pop_index {
				if lowest_pop_index == 0 {
					self.priority_ids.clear();
				} else {
					self.priority_ids.split_off(&WidgetId::from_index(lowest_pop_index));
				}
			}

			Some(widget)
		} else {
			//we're removing something in the middle of the collection
			//put a place holder so indices and ids dont have to change
			let widget = mem::replace(&mut self.widgets[index], None)?;

			//queue it for the next_id method
			self.priority_ids.insert(id);

			Some(widget)
		}
	}

	pub fn set_parent(&mut self, id: WidgetId) -> Option<()> {
		let index = id.to_index();
		let Some(Some(widget)) = self.widgets.get_mut(index) else {
			return None;
		};

		if let Some(previous_parent) = mem::replace(&mut widget.parent_id, Some(id)) {
			//TODO: put expects here insteada' unwraps!
			let child_indices = &mut (&mut self.widgets[previous_parent.to_index()]).as_mut().unwrap().child_indices;

			child_indices.remove(*child_indices.iter().find(|child_index| **child_index == index).unwrap());
		}

		Some(())
	}

	pub fn widget(&self, id: WidgetId) -> Option<&Widget> {
		self.widgets.get(id.to_index())?.as_ref()
	}

	pub fn widget_mut(&mut self, id: WidgetId) -> Option<&mut Widget> {
		let index = id.to_index();

		match self.widgets.get_mut(index) {
			Some(option) => option.as_mut(),

			_ => None,
		}
	}

	pub fn widgets_mut(&mut self, ids: &[WidgetId]) -> Option<WidgetsIterMut> {
		let mut indices = Vec::<usize>::with_capacity(ids.len());

		for id in ids {
			let index = id.to_index();

			//return via Try if one of the widgets with the given id does not exist
			self.widgets.get(index)?.as_ref()?;
			indices.push(index);
		}

		Some(WidgetsIterMut {
			base: self,
			index: 0,
			indices,
		})
	}
}

pub struct WidgetsIterMut<'a> {
	base: &'a mut WidgetBase,
	index: usize,
	indices: Vec<usize>,
}

impl<'a> ExactSizeIterator for WidgetsIterMut<'a> { }

impl<'a> Iterator for WidgetsIterMut<'a> {
	type Item = &'a mut Widget;

	fn next(&mut self) -> Option<Self::Item> {
		//TODO: unwrap -> expect
		let index = *self.indices.get(self.index)?;
		self.index += 1;

		//TODO: VERIFY SAFETY!
		return Some(unsafe { &mut *((&mut self.base.widgets[index]).as_mut().unwrap() as *mut _) });
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let remain = self.indices.len().saturating_sub(self.index);

		(remain, Some(remain))
	}
}

#[test]
fn test() {
	//TODO: test the safety of WidgetsIterMut
	use super::builder::WidgetBuilder;

	let mut db = WidgetBase::new();
	let wb = WidgetBuilder::new().with_size([640., 480.]);
	let root = db.insert(wb.try_clone().unwrap());

	
}

pub trait BuildWidget {
	fn build_widget(self, id: WidgetId) -> Widget;
}