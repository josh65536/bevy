use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum AppState {
    A,
    B,
}

fn main() {
    App::new()
        .add_state(AppState::A)
        .add_system_set(
            SystemSet::on_enter(AppState::B).after("change").with_system(on_state_change)
        )
        .add_system_set(
            SystemSet::on_update(AppState::B).label("b").with_system(in_state_b)
        )
        .add_system(change_state.label("change").after("b"))
        .run();
}

fn change_state(mut state: ResMut<State<AppState>>) {
    println!("change_state");
    state.set(AppState::B).ok();
}

fn on_state_change() {
    println!("on_state_change");
}

fn in_state_b() {
    println!("in_state_b");
}