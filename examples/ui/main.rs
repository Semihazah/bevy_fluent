use crate::{
    components::{ColorMaterials, DefaultFont, Locales},
    locales::{de, en, ru},
    systems::{load, menu},
};
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_fluent::prelude::*;

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::ERROR,
            filter: "bevy_fluent=trace".to_string(),
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "examples/ui/assets".to_string(),
            watch_for_changes: false,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .insert_resource(
            Locales::new(de::DE)
                .with_default(en::US)
                .with(ru::BY)
                .with(ru::RU),
        )
        .init_resource::<ColorMaterials>()
        .init_resource::<DefaultFont>()
        .add_state(GameState::Load)
        .add_system_set(SystemSet::on_enter(GameState::Load).with_system(load::setup))
        .add_system_set(SystemSet::on_update(GameState::Load).with_system(load::loading))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu::interaction)
                .with_system(menu::next)
                .with_system(menu::previous),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(menu::cleanup))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Load,
    Menu,
}

mod components;
mod locales;
mod pathfinders;
mod systems;
mod to_sentence_case;
