use std::sync::Arc;

use crate::{system::DisplayId, window::Window, AppState};
use item::Item;
use crate::{SystemResult, SystemError};
use item_section::ItemSection;
use parking_lot::Mutex;

pub mod component;
pub mod create;
pub mod item;
pub mod item_section;

// A class for working with title bars (rendered by default at the top of the screen during work mode)
#[derive(Clone, Debug)]
pub struct Bar {
	pub window: Window,
	pub display_id: DisplayId,
	pub left: ItemSection,
	pub center: ItemSection,
	pub right: ItemSection,
}

impl Default for Bar {
	fn default() -> Self {
		Self {
			window: Window::new(),
			display_id: DisplayId::default(),
			left: ItemSection::default(),
			center: ItemSection::default(),
			right: ItemSection::default(),
		}
	}
}

impl Bar {
	// Finds and returns the item at the position or None
	// TODO: Are positions here in pixels?
	pub fn item_at_pos(&self, x: i32) -> Option<&Item> {
		// Create a vector from the zones of every section in all three zones in this bar to iterate over
		for section in vec![&self.left, &self.center, &self.right] {
			// If the position is contained in the section
			if section.left <= x && x <= section.right {
				// Iterate over the items in the section
				for item in section.items.iter() {
					// If the position is contained in the Item, return it
					if item.left <= x && x <= item.right {
						return Some(item);
					}
				}
			}
		}

		None
	}

	// Updates the height of this bar
	pub fn change_height(&self, height: i32) -> SystemResult {
		let nwin = self.window.get_native_window();
		let mut rect = nwin.get_rect()?;

		if rect.top == 0 {
			rect.bottom = height;
		} else {
			// TODO: This math is a bit suspicious
			rect.top = rect.bottom - height;
		}

		nwin.set_window_pos(rect, None, None).map_err(|e| SystemError::Unknown(e))
	}
}

// A global function to close (derender) all rendered bars
pub fn close_all(state_arc: Arc<Mutex<AppState>>) {
	let mut windows = Vec::new();

	for d in state_arc.lock().displays.iter_mut() {
		if let Some(b) = d.appbar.as_ref() {
			windows.push(b.window.clone())
		}
		d.appbar = None;
	}

	for w in windows {
		w.close();
	}
}
