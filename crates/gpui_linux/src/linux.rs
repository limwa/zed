mod dispatcher;
mod headless;
mod keyboard;
mod platform;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod text_system;
#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11")]
mod x11;

#[cfg(any(feature = "wayland", feature = "x11"))]
mod xdg_desktop_portal;

pub use dispatcher::*;
pub(crate) use headless::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

use std::{ops::Range, rc::Rc};

use gpui::{DispatchEventResult, PlatformInput, PlatformInputHandler};

pub(crate) trait MarkedTextInput {
    fn marked_text_range(&mut self) -> Option<Range<usize>>;
    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str);
}

impl MarkedTextInput for PlatformInputHandler {
    fn marked_text_range(&mut self) -> Option<Range<usize>> {
        self.marked_text_range()
    }

    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.replace_text_in_range(replacement_range, text);
    }
}

pub(crate) fn delete_marked_text(input_handler: &mut impl MarkedTextInput) -> bool {
    let Some(marked_range) = input_handler.marked_text_range() else {
        return false;
    };

    input_handler.replace_text_in_range(Some(marked_range), "");
    true
}

pub(crate) fn should_cancel_preedit(input: &PlatformInput, result: &DispatchEventResult) -> bool {
    !result.propagate
        && matches!(input, PlatformInput::KeyDown(event) if event.keystroke.key_char.is_none())
}

/// Returns the default platform implementation for the current OS.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    #[cfg(feature = "x11")]
    use anyhow::Context as _;

    if headless {
        return Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        });
    }

    match gpui::guess_compositor() {
        #[cfg(feature = "wayland")]
        "Wayland" => Rc::new(LinuxPlatform {
            inner: WaylandClient::new(),
        }),

        #[cfg(feature = "x11")]
        "X11" => Rc::new(LinuxPlatform {
            inner: X11Client::new()
                .context("Failed to initialize X11 client.")
                .unwrap(),
        }),

        "Headless" => Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        }),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{KeyDownEvent, KeyUpEvent, Keystroke, Modifiers};

    #[derive(Default)]
    struct FakeMarkedTextInput {
        marked_range: Option<Range<usize>>,
        replacements: Vec<(Option<Range<usize>>, String)>,
    }

    impl MarkedTextInput for FakeMarkedTextInput {
        fn marked_text_range(&mut self) -> Option<Range<usize>> {
            self.marked_range.clone()
        }

        fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
            self.replacements
                .push((replacement_range, text.to_string()));
            self.marked_range = None;
        }
    }

    #[test]
    fn test_delete_marked_text_replaces_marked_range_with_empty_string() {
        let mut input = FakeMarkedTextInput {
            marked_range: Some(2..3),
            ..Default::default()
        };

        assert!(delete_marked_text(&mut input));
        assert_eq!(input.replacements, vec![(Some(2..3), String::new())]);
        assert_eq!(input.marked_range, None);
    }

    #[test]
    fn test_delete_marked_text_skips_when_nothing_is_marked() {
        let mut input = FakeMarkedTextInput::default();

        assert!(!delete_marked_text(&mut input));
        assert!(input.replacements.is_empty());
    }

    #[test]
    fn test_should_cancel_preedit_for_consumed_dead_key_shortcut() {
        let input = PlatformInput::KeyDown(KeyDownEvent {
            keystroke: Keystroke {
                modifiers: Modifiers::control(),
                key: "\\".into(),
                key_char: None,
            },
            is_held: false,
            prefer_character_input: false,
        });

        assert!(should_cancel_preedit(
            &input,
            &DispatchEventResult {
                propagate: false,
                default_prevented: false,
            }
        ));
    }

    #[test]
    fn test_should_not_cancel_preedit_for_text_or_unconsumed_input() {
        let dead_key_input = PlatformInput::KeyDown(KeyDownEvent {
            keystroke: Keystroke {
                modifiers: Modifiers::control(),
                key: "\\".into(),
                key_char: None,
            },
            is_held: false,
            prefer_character_input: false,
        });
        let text_input = PlatformInput::KeyDown(KeyDownEvent {
            keystroke: Keystroke {
                modifiers: Modifiers::default(),
                key: "~".into(),
                key_char: Some("~".into()),
            },
            is_held: false,
            prefer_character_input: false,
        });
        let key_up_input = PlatformInput::KeyUp(KeyUpEvent {
            keystroke: Keystroke {
                modifiers: Modifiers::control(),
                key: "\\".into(),
                key_char: None,
            },
        });

        assert!(!should_cancel_preedit(
            &dead_key_input,
            &DispatchEventResult {
                propagate: true,
                default_prevented: false,
            }
        ));
        assert!(!should_cancel_preedit(
            &text_input,
            &DispatchEventResult {
                propagate: false,
                default_prevented: false,
            }
        ));
        assert!(!should_cancel_preedit(
            &key_up_input,
            &DispatchEventResult {
                propagate: false,
                default_prevented: false,
            }
        ));
    }
}
