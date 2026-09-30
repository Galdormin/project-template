//! Fonts used by the UI.

use bevy::{prelude::*, text::FontSourceTemplate};

/// A font shipped with the game, in `assets/fonts`.
#[derive(Clone, Copy, Debug, Default)]
pub enum CustomFont {
    /// Bevy's default font.
    #[default]
    Default,
    Monogram,
}

impl CustomFont {
    fn path(self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::Monogram => Some("fonts/monogram-extended.ttf"),
        }
    }

    /// Sets the font of a text entity, keeping the rest of its [`TextFont`].
    pub fn scene(self) -> impl Scene {
        self.path()
            .map(|path| bsn! { TextFont { font: FontSourceTemplate::Handle({path}) } })
    }
}

/// Uses the Monogram font.
#[derive(SceneComponent, Default, Clone)]
pub struct MonogramFont;

impl MonogramFont {
    fn scene() -> impl Scene {
        CustomFont::Monogram.scene()
    }
}
