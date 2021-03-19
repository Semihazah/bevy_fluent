//! Plugins module
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use crate::{
    systems::{check_assets, load_assets, take_snapshot},
    FluentAsset, FluentAssetLoader, FluentSettings, FluentState,
};
#[cfg(not(feature = "implicit"))]
use crate::{LocaleAssets, LocaleAssetsLoader};
use bevy::prelude::{stage::PRE_UPDATE, *};

static STATE: &str = "fluent";

/// Adds support for Fluent file loading to Apps
#[derive(Default)]
pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.resources_mut()
            .get_or_insert_with(FluentSettings::default);
        #[cfg(not(feature = "implicit"))]
        app.init_asset_loader::<LocaleAssetsLoader>()
            .add_asset::<LocaleAssets>();
        app.init_asset_loader::<FluentAssetLoader>()
            .add_asset::<FluentAsset>()
            .add_resource(State::new(FluentState::LoadAssets))
            .add_stage_before(PRE_UPDATE, STATE, StateStage::<FluentState>::default())
            .on_state_enter(STATE, FluentState::LoadAssets, load_assets.system())
            .on_state_update(STATE, FluentState::LoadAssets, check_assets.system())
            .on_state_enter(STATE, FluentState::TakeSnapshot, take_snapshot.system());
    }
}
