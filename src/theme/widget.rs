//! Reusable scene components for common widgets.

use bevy::prelude::*;

use crate::theme::{interaction::InteractionPalette, palette::*};

/// A root UI node that fills the window and centers its content.
#[derive(SceneComponent, Default, Clone)]
pub struct UiRoot;

impl UiRoot {
    fn scene() -> impl Scene {
        bsn! {
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
            }
            // Don't block picking events for other UI roots.
            Pickable::IGNORE
        }
    }
}

/// A simple header label. Bigger than [`Label`].
#[derive(SceneComponent, Default, Clone)]
pub struct Header;

impl Header {
    fn scene() -> impl Scene {
        bsn! {
            Text
            TextFont { font_size: px(40) }
            TextColor(HEADER_TEXT)
        }
    }
}

/// A simple text label.
#[derive(SceneComponent, Default, Clone)]
pub struct Label;

impl Label {
    fn scene() -> impl Scene {
        bsn! {
            Text
            TextFont { font_size: px(24) }
            TextColor(LABEL_TEXT)
        }
    }
}

/// A button with text. Attach its action with `on(...)`.
#[derive(SceneComponent, Default, Clone)]
#[scene(ButtonProps)]
pub struct UiButton;

#[derive(Default)]
pub struct ButtonProps {
    pub text: String,
    pub size: ButtonSize,
}

#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    /// A large rounded button.
    #[default]
    Large,
    /// A small square button.
    Small,
}

impl UiButton {
    fn scene(props: ButtonProps) -> impl Scene {
        let (width, height, border_radius) = match props.size {
            ButtonSize::Large => (px(380), px(80), BorderRadius::MAX),
            ButtonSize::Small => (px(30), px(30), BorderRadius::ZERO),
        };
        bsn! {
            Button
            Node {
                width: {width},
                height: {height},
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: {border_radius},
            }
            BackgroundColor(BUTTON_BACKGROUND)
            InteractionPalette {
                none: BUTTON_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            }
            Children [(
                #ButtonText
                Text({props.text})
                TextFont { font_size: px(40) }
                TextColor(BUTTON_TEXT)
                // Don't bubble picking events from the text up to the button.
                Pickable::IGNORE
            )]
        }
    }
}
