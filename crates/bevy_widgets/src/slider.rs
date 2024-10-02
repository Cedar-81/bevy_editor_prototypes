use bevy::{color::palettes::tailwind, prelude::*};
use bevy_editor_camera::*;

#[derive(Component, Default, Clone)]
// #[require(
//     Node,
//     BackgroundColor(slide_background_color_init),
//     Style(slide_style_init)
// )]
pub struct Slider {
    pub value: f32,
}

#[derive(Component, Default, Clone)]
#[require(Node, Style, BackgroundColor, BorderRadius, BorderColor)]
struct SliderHandle;

#[derive(Component, Default, Clone)]
#[require(
    Node,
    Style(slide_style_init),
    BackgroundColor(slide_background_color_init),
    BorderRadius,
    BorderColor
)]
struct Slide;

/// This component serves as an easy means to customize the sliders handle and slide components during initialization
// #[derive(Component, Debug, Clone, Default)]
// pub struct SliderComponentsStyle {
//     pub slide_height: Val,
//     pub slide_width: Val,
//     pub slide_border_color: BorderColor,
//     pub slide_border_radius: BorderRadius,
//     pub slide_border: UiRect,
//     pub slide_background_color: BackgroundColor,

//     //handle styles
//     pub handle_height: Val,
//     pub handle_width: Val,
//     pub handle_border_color: BorderColor,
//     pub handle_border: UiRect,
//     pub handle_border_radius: BorderRadius,
//     pub handle_background_color: BackgroundColor,
// }

// fn slider_component_style_init() -> SliderComponentsStyle {
//     SliderComponentsStyle { ..default() }
// }

fn slide_style_init() -> Style {
    Style {
        height: Val::Px(106.),
        width: Val::Px(308.),
        ..default()
    }
}

fn slide_background_color_init() -> BackgroundColor {
    BackgroundColor(tailwind::AMBER_600.into())
}

fn spawn_slider_components(mut commands: Commands, command_query: Query<Entity, With<Slider>>) {}
