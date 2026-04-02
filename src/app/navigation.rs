use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Button, Stack};

#[derive(Clone)]
pub(crate) struct LauncherNavigator {
    stack: Stack,
    section_buttons: Rc<RefCell<Vec<(String, Button)>>>,
}

impl LauncherNavigator {
    pub(crate) fn new(stack: &Stack) -> Self {
        Self {
            stack: stack.clone(),
            section_buttons: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(crate) fn register_sidebar_button(&self, section_id: &str, button: &Button) {
        self.section_buttons
            .borrow_mut()
            .push((section_id.to_string(), button.clone()));
    }

    pub(crate) fn activate_section(&self, section_id: &str) {
        self.stack
            .set_visible_child_name(Self::page_name_for_section(section_id));

        for (candidate_id, button) in self.section_buttons.borrow().iter() {
            if candidate_id == section_id {
                button.add_css_class("is-active");
            } else {
                button.remove_css_class("is-active");
            }
        }
    }

    fn page_name_for_section(section_id: &str) -> &'static str {
        match section_id {
            "home" | "dashboard" | "start" => "dashboard",
            _ => "all-apps",
        }
    }
}
