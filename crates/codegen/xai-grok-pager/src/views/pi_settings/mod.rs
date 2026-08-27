//! Legacy grok-pi settings implementation.
//!
//! The normal `/settings` and F2 paths now route through the canonical
//! [`crate::views::settings_modal`] surface so grok-pi matches Grok exactly.
//! This module remains isolated for compatibility with the existing modal
//! seams and tests; Pi-specific registry entries and actions are maintained by
//! the canonical settings flow.

mod actions;
mod input;
pub mod layout;
mod render;
mod state;

#[cfg(test)]
mod tests;

pub use input::{handle_key, handle_mouse, handle_paste};
pub use render::render_pi_settings;
pub use state::{MODAL_TITLE, ModeKind, Outcome, PiSettingsState, Row};
