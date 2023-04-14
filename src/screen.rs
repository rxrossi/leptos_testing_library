use leptos::{IntoView, Scope};
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use super::setup_dom::setup_dom;

pub struct Screen {
    dom: web_sys::Element,
}

impl Screen {
    pub fn new<F, N>(view: F) -> Screen
    where
        F: FnOnce(Scope) -> N + 'static,
        N: IntoView,
    {
        Screen {
            dom: setup_dom(view),
        }
    }

    /// Get a button by its text
    pub fn button(&self, text: &str) -> HtmlButtonElement {
        let buttons = self.get_elements::<HtmlButtonElement>("button");
        buttons
            .iter()
            .find(|button| button.text_content().unwrap() == text)
            .expect("Button not found")
            .clone()
            .unchecked_into()
    }

    /// Get a input by its placeholder or label (by label is not supported yet)
    pub fn input(&self, selector: &str) -> HtmlInputElement {
        self.get_elements::<web_sys::HtmlInputElement>("input")
            .into_iter()
            .find(|element| {
                // TODO: Add by label as well
                return element.placeholder() == selector;
            })
            .unwrap()
            .clone()
            .unchecked_into()
    }

    /// Get elements by text content
    pub fn by_text(&self, text: &str) -> Vec<web_sys::Element> {
        self.get_elements::<web_sys::Element>("*")
            .into_iter()
            .filter(|element| {
                let text_content = element.text_content().unwrap();
                let child_count = element.child_nodes().length();

                return text_content == text && child_count == 1;
            })
            .collect()
    }

    pub fn get_elements<T>(&self, query_selector: &str) -> Vec<T>
    where
        T: wasm_bindgen::JsCast,
    {
        let node_list = self
            .dom
            .query_selector_all(query_selector)
            .expect("Could not query the elements");

        let mut nodes: Vec<T> = vec![];

        for i in 0..node_list.length() {
            let element = node_list
                .get(i)
                .expect(&format!("element does not exist {}", i));

            nodes.push(element.unchecked_into());
        }

        return nodes;
    }
}

