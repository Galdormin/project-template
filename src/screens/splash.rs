//! A splash screen that plays briefly at startup.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{AppSystems, asset_tracking::LoadResource, screens::Screen, theme::widget::UiRoot};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<SplashAssets>();

    // Spawn splash screen once its images are loaded.
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(
        Update,
        spawn_splash_screen
            .run_if(in_state(Screen::Splash).and_then(resource_added::<SplashAssets>)),
    );

    // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSystems::TickTimers),
            (apply_fade_in_out, load_splash_image).in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash).and_then(resource_exists::<SplashAssets>)),
    );

    // Move to the next splash when the current one is over, or if the player hits escape.
    app.add_systems(
        Update,
        advance_splash
            .run_if(
                in_state(Screen::Splash)
                    .and_then(input_just_pressed(KeyCode::Escape).or_else(splash_finished)),
            )
            .in_set(AppSystems::Update),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
#[require(ImageNode)]
enum SplashBackground {
    #[default]
    Bevy,
    Jam,
    Studio,
}

impl SplashBackground {
    fn image(&self, assets: &SplashAssets) -> Handle<Image> {
        match self {
            Self::Bevy => assets.bevy.clone(),
            Self::Jam => assets.jam.clone(),
            Self::Studio => assets.studio.clone(),
        }
    }

    fn next(&self) -> Option<Self> {
        match self {
            Self::Bevy => Some(Self::Jam),
            Self::Jam => Some(Self::Studio),
            Self::Studio => None,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct SplashAssets {
    #[dependency]
    bevy: Handle<Image>,
    #[dependency]
    jam: Handle<Image>,
    #[dependency]
    studio: Handle<Image>,
}

impl FromWorld for SplashAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let load = |path: &'static str| {
            assets
                .load_builder()
                .with_settings(
                    // This should be an embedded asset for instant loading, but that is
                    // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                    |settings: &mut ImageLoaderSettings| {
                        // Make an exception for the splash images in case
                        // `ImagePlugin::default_nearest()` is used for pixel art.
                        settings.sampler = ImageSampler::linear();
                    },
                )
                .load(path)
        };
        Self {
            bevy: load("images/splash/bevy_splash.png"),
            jam: load("images/splash/jam_splash.png"),
            studio: load("images/splash/studio_splash.png"),
        }
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn new(total_duration: f32, fade_duration: f32) -> Self {
        Self {
            total_duration,
            fade_duration,
            t: 0.0,
        }
    }

    fn reset(&mut self) {
        self.t = 0.0;
    }

    fn is_finished(&self) -> bool {
        self.t >= self.total_duration
    }

    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn spawn_splash_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #SplashScreen
        @UiRoot
        BackgroundColor(SPLASH_BACKGROUND_COLOR)
        DespawnOnExit::<Screen>(Screen::Splash)
        Children [(
            #SplashImage
            Node { width: percent(70) }
            SplashBackground::default()
            ImageNodeFadeInOut::new(SPLASH_DURATION_SECS, SPLASH_FADE_DURATION_SECS)
        )]
    });
}

fn load_splash_image(
    splash_assets: Res<SplashAssets>,
    splash_images: Query<(&mut ImageNode, &SplashBackground), Changed<SplashBackground>>,
) {
    for (mut node, splash) in splash_images {
        node.image = splash.image(&splash_assets);
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

fn splash_finished(splash: Query<&ImageNodeFadeInOut, With<SplashBackground>>) -> bool {
    splash.iter().any(ImageNodeFadeInOut::is_finished)
}

fn advance_splash(
    splash: Single<(&mut SplashBackground, &mut ImageNodeFadeInOut)>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let (mut background, mut fade) = splash.into_inner();
    match background.next() {
        Some(next) => {
            *background = next;
            fade.reset();
        }
        None => next_screen.set(Screen::Title),
    }
}
