//! The main Bevy Editor application.
//!
//! This crate contains a standalone application that can be used to edit Bevy scenes and debug Bevy games.
//! Virtually all of the underlying logic and functionality of the editor should be backed by the assorted crates in the `bevy_editor` workspace;
//! this crate is simply responsible for orchestrating those crates and providing a user interface for them.
//!
//! The exact nature of this crate will be in flux for a while:
//!
//! - Initially, this will be a standard Bevy application that simply edits scenes with `DefaultPlugins`.
//! - Then, it will be a statically linked plugin that can be added to any Bevy game at compile time,
//!     which transforms the user's application into an editor that runs their game.
//! - Finally, it will be a standalone application that communicates with a running Bevy game via the Bevy Remote Protocol.

use bevy::{color::palettes::tailwind, prelude::*};
// use bevy_editor_camera::*;
pub use bevy_widgets::prelude::*;
use slider::Slider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let slider = commands
        .spawn((
            Node::DEFAULT,
            Slider { value: 0. },
            Style {
                height: Val::Px(20.),
                width: Val::Px(30.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BorderColor(tailwind::AMBER_600.into()),
            BackgroundColor(tailwind::AMBER_600.into()),
        ))
        .id();
    let button = commands
        .spawn(ButtonBundle {
            style: Style {
                height: Val::Px(30.),
                width: Val::Px(10.),
                ..default()
            },
            background_color: BackgroundColor(tailwind::BLUE_400.into()),
            ..default()
        })
        .id();
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: BorderColor(tailwind::BLUE_400.into()),
            ..default()
        })
        .add_child(slider)
        .add_child(button);
}
