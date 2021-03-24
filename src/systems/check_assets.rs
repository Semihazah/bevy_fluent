use crate::{components::Handles, FluentState};
use bevy::{
    app::{AppExit, Events},
    asset::LoadState,
    prelude::*,
    utils::tracing::{self, instrument},
};

#[instrument(fields(handles = handles.len(), state = ?*state), skip(asset_server, events))]
pub(crate) fn check_assets(
    asset_server: Res<AssetServer>,
    mut events: ResMut<Events<AppExit>>,
    handles: Res<Handles>,
    mut state: ResMut<State<FluentState>>,
) {
    trace!("call");
    match asset_server.get_group_load_state(handles.iter().map(|handle| handle.id)) {
        LoadState::Failed => events.send(AppExit),
        LoadState::Loaded => {
            debug!(load_state = ?LoadState::Loaded);
            state.set_next(FluentState::TakeSnapshot).unwrap();
        }
        _ => {}
    }
}
