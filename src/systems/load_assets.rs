use crate::{components::Handles, FluentSettings};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use std::path::Path;

#[instrument(fields(locales_folder = %settings.locales_folder), skip(commands, asset_server, settings))]
pub(crate) fn load_assets(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    settings: Res<FluentSettings>,
) {
    trace!("call");
    let path = Path::new(&settings.locales_folder);
    let handles = asset_server
        .load_folder(path)
        .expect("load assets from root locales directory");
    commands.insert_resource(Handles(handles));
}
