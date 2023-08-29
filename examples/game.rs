use std::time::Duration;

use bevy::{prelude::*, app::ScheduleRunnerPlugin};
use bevy_cleanup::{Cleanup, AddStateCleanup};

fn main() {
    // Boring setup stuff
    App::new()
        .add_plugins(
            MinimalPlugins.set(
                // Just to make the println's slower
                ScheduleRunnerPlugin::run_loop(Duration::from_millis(500)),
            )
        )
        //
        .add_state::<AppState>()
        // When transitioning *out* of `AppState::Menu`, all entities with `CleanupMenu`
        // And all children of that entity are recursively removed. 
        .add_state_cleanup::<_, CleanupMenu>(AppState::Menu)
        // Same for `AppState::Game` and `CleanupGame`.
        .add_state_cleanup::<_, CleanupGame>(AppState::Game)
        //
        // Our game logic here
        .add_systems(
            OnEnter(AppState::Menu),
            (setup_menu, apply_deferred, print_all_entity_names).chain(),
        )
        .add_systems(
            OnEnter(AppState::Game),
            (setup_game, apply_deferred, print_all_entity_names).chain(),
        )
        .add_systems(
            Update,
            (
                update_menu.run_if(in_state(AppState::Menu)),
                damage_over_time.run_if(in_state(AppState::Game)),
            )
        )
        .run()
}

// You probably want to keep your state variants and Cleanup types together.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

// Use the `Cleanup` derive macro to automatically implement the `Cleanup` trait.
#[derive(Debug, Component, Cleanup)]
pub struct CleanupMenu;

#[derive(Debug, Component, Cleanup)]
pub struct CleanupGame;

// Debug system to print out all entities with a name in the world

fn print_all_entity_names(query: Query<&Name>) {
    println!("Entities in world:");
    for name in &query {
        println!(" - {}", name);
    }
}

// Menu stuff

#[derive(Debug, Component)]
pub struct MenuUi;

fn setup_menu(mut commands: Commands) {
    // Here is where we spawn in entities which are associated with the `AppState::Menu` state.
    // The convention is to have a name and the cleanup type at the top of the bundle tuple.
    commands.spawn((
        Name::new("Menu UI"),
        // This component will make the entity be removed when *exiting* the menu state.
        // This is defined by `.add_state_cleanup` during app build.
        CleanupMenu,
        // The rest of our components go down below.
        MenuUi,
    ));

    println!("Set up menu - game will start in 1 second");
}

fn update_menu(time: Res<Time>, mut next_state: ResMut<NextState<AppState>>) {
    // Automatically move to the game state after 1 second
    if time.elapsed_seconds() > 1.0 {
        // Imagine that our player has just pressed the "start game" button.
        //
        // On this transition, all entities with CleanupMenu are removed.
        next_state.set(AppState::Game);
    }
}

// Game stuff

#[derive(Debug, Component)]
pub struct Health(pub f32);

#[derive(Debug, Component)]
pub struct DamageOverTime(pub f32);

fn setup_game(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        CleanupGame,
        //
        Health(1.0),
        // 0.25 damage per sec; 4 seconds to kill the player
        DamageOverTime(0.25),
    ));

    println!("Set up game");
}

fn damage_over_time(
    time: Res<Time>,
    mut query: Query<(&mut Health, &DamageOverTime)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (mut health, dot) in &mut query {
        // The player will take damage over time
        health.0 -= dot.0 * time.delta_seconds();
        println!("Player health is {}", health.0);
        if health.0 <= 0.0 {
            // Wow, this player is terrible at our (unwinnable) game and died!
            // Let's signal a "game over" by kicking him to the main menu.
            //
            // On this transition, all entities with CleanupGame are removed.
            println!("Player died! Switching to main menu");
            next_state.set(AppState::Menu);
        }
    }
}
