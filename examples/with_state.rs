use bevy::prelude::{stage::UPDATE, *};
use bevy_fluent::{prelude::*, utils::BundleExt, Snapshot};
use unic_langid::langid;

static GAME_STATE: &str = "game";

pub fn main() {
    App::build()
        .add_resource(FluentSettings::default().with_default_locale(langid!("ru-RU")))
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_resource(State::new(GameState::Initialize))
        .add_stage_before(UPDATE, GAME_STATE, StateStage::<GameState>::default())
        .on_state_update(GAME_STATE, GameState::Initialize, check_fluent.system())
        .on_state_enter(GAME_STATE, GameState::Play, localized_hello_world.system())
        .run();
}

fn check_fluent(fluent_state: Res<State<FluentState>>, mut game_state: ResMut<State<GameState>>) {
    if **fluent_state == FluentState::Done {
        game_state.set_next(GameState::Play).unwrap();
    }
}

fn localized_hello_world(snapshot: Res<Snapshot>, mut done: Local<bool>) {
    if *done {
        return;
    }
    *done = true;
    let request = Request::builder().id("hello-world").build();
    let hello_world = snapshot.content(&request).unwrap();
    println!("{}", hello_world);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GameState {
    Initialize,
    Play,
}
