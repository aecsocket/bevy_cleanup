# bevy_cleanup

[![crates.io](https://img.shields.io/crates/v/bevy_cleanup.svg)](https://crates.io/crates/bevy_cleanup)

Providers helpers for using the cleanup design pattern in Bevy, where entities are automatically
removed after a state transition, depending on any `Cleanup` marker components they have.

```rs
use bevy::prelude::*;
use bevy_cleanup::{Cleanup, AddStateCleanup};

// Set up your States enum and keep your Cleanup component types close by

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

#[derive(Component, Cleanup)]
struct CleanupMenu;

#[derive(Component, Cleanup)]
struct CleanupGame;

// Set up your App

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_state_cleanup::<_, CleanupMenu>(AppState::Menu)
        .add_state_cleanup::<_, CleanupGame>(AppState::Game)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .run();
}

fn setup_menu(mut commands: Commands) {
    // When spawning an entity, give it one of your `Cleanup` component types
    // Typically, you put the Name and Cleanup types at the top of the component tuple
    commands.spawn((
        Name::new("Menu entity"),
        CleanupMenu,
        // everything else...
    ));
}

// When you want to switch from Menu to Game...
fn switch_to_game(mut next_state: ResMut<NextState<AppState>>) {
    // After this, any entities with the cleanup component of the current state (CleanupMenu)
    // will be automatically recursively despawned.
    next_state.set(AppState::Game);
}
```
