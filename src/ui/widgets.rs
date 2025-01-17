//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::{
    assets::{FontKey, HandleMap},
    //AppSet,
};

use super::palette::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, set_button_font);
}

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, font_size: f32, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn tool_bar(&mut self) -> EntityCommands;

    fn text(&mut self, text: impl Into<String>) -> EntityCommands;
}

#[derive(Debug, Component)]
struct ButtonText;

fn set_button_font(
    mut q_text: Query<&mut Text, Added<ButtonText>>,
    font_handles: Res<HandleMap<FontKey>>,
) {
    for mut text in &mut q_text {
        //for section in &mut text.sections {
            //section.style.font = font_handles[&FontKey::GeoFont].clone_weak();
        //}
    }
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, font_size: f32, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            Button{
                ..default()
            },
            BackgroundColor(NODE_BACKGROUND),   
            //UiRect::axes(Val::Vh(5.), Val::Vh(1.)),
            TextLayout::new_with_justify(JustifyText::Center)
        ));
        entity.with_children(|children| {
            children.spawn(( 
                ButtonText,
                Name::new("Button Text"),
            ));
        });
        entity
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Px(500.0),
                height: Px(65.0),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn(( 
                Name::new("Header Text"),

            ));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            Node {
                width: Px(500.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn(( 
                Name::new("Label Text"),
            ));
        });
        entity
    }

    fn text(&mut self, text: impl Into<String>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Label Text"),
        ));
        entity
    }

    fn tool_bar(&mut self) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Tool bar"),
            Node {
                width: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Baseline,
                flex_direction: FlexDirection::Row,
                column_gap: Px(10.0),
                position_type: PositionType::Relative,
                padding: UiRect::all(Val::Vh(1.0)),
                ..default()
            },
        ));
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_center_root(&mut self) -> EntityCommands;
    fn ui_top_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_center_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Centering UI Root"),
            Node{
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
        
            },
        ))
    }
    fn ui_top_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Top UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
/*
impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
*/

