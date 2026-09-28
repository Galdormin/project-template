//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    menus::Menu,
    screens::Screen,
    theme::widget::{UiButton, UiRoot},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #MainMenu
        @UiRoot
        GlobalZIndex(2)
        DespawnOnExit::<Menu>(Menu::Main)
        Children [
            @UiButton { @text: "Play" } on(enter_loading_or_gameplay_screen),
            @UiButton { @text: "Settings" } on(open_settings_menu),
            @UiButton { @text: "Credits" } on(open_credits_menu),
            {exit_button()},
        ]
    });
}

#[cfg(not(target_family = "wasm"))]
fn exit_button() -> impl SceneList {
    bsn_list![@UiButton { @text: "Exit" } on(exit_app)]
}

#[cfg(target_family = "wasm")]
fn exit_button() -> impl SceneList {
    bsn_list![]
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
