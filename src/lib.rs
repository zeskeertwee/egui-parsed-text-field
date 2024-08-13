use std::fmt::Debug;
use std::str::FromStr;
use egui::{Color32, Response, TextEdit, Ui, Widget};
use log::trace;

pub struct ParsedTextFieldState<T: FromStr> {
    pub text: String,
    pub parsed: Option<T>,
    last_failed: bool,
}

pub struct ParsedTextField<'a, T: FromStr> {
    state: &'a mut ParsedTextFieldState<T>
}

impl<'a, T: FromStr> Widget for ParsedTextField<'a, T> where <T as FromStr>::Err: Debug {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.add(TextEdit::singleline(&mut self.state.text).text_color(if self.state.last_failed { Color32::RED } else { Color32::WHITE }));

        if response.changed() {
            match self.state.text.parse::<T>() {
                Ok(v) => {
                    self.state.last_failed = false;
                    self.state.parsed = Some(v);
                },
                Err(e) => {
                    trace!("Invalid ParsedTextField input: {}: {e:?}", self.state.text);
                    self.state.parsed = None;
                    self.state.last_failed = true;
                }
            }
        }

        response
    }
}

impl<'a, T: FromStr> ParsedTextField<'a, T> {
    pub fn new(state: &'a mut ParsedTextFieldState<T>) -> Self {
        Self {
            state
        }
    }
}

impl<T: FromStr + Default + ToString> Default for ParsedTextFieldState<T> {
    fn default() -> Self {
        Self {
            text: T::default().to_string(),
            parsed: Some(T::default()),
            last_failed: false,
        }
    }
}