#![warn(clippy::all)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

#[cfg(feature = "derive")]
pub use bevy_cleanup_derive::Cleanup;

/// The trait used to denote that a component is a "cleanup marker".
///
/// This trait is never queried for by itself, but is a bound for
/// [`AddStateCleanup::add_state_cleanup`], just to make sure you don't accidentally mark the wrong
/// component as a cleanup component.
///
/// This marker trait is typically automatically derived using the `Cleanup` derive macro (it's
/// simply an empty implementation block). You would typically keep your `States` enum and all
/// `Cleanup`-deriving types close together.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_cleanup::Cleanup;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
/// enum AppState {
///     #[default]
///     Menu,
///     Game,
/// }
///
/// #[derive(Component, Cleanup)]
/// struct CleanupMenu;
///
/// #[derive(Component, Cleanup)]
/// struct CleanupGame;
/// ```
pub trait Cleanup: Component {}

/// Allows using [`Self::add_state_cleanup`].
pub trait AddStateCleanup {
    /// When the state `variant` is exited ([`OnExit`]), all entities which have component `C`
    /// will be recursively despawned.
    /// 
    /// You do not have to add a state cleanup to *every* `S` variant, just the ones you want to
    /// despawn entities for when you exit that variant. So if you know you won't be adding any
    /// entities during a state (e.g. a `LoadGame` state), then don't bother adding state cleanup
    /// to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_cleanup::{Cleanup, AddStateCleanup};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
    /// enum AppState {
    ///     #[default]
    ///     Menu,
    ///     Game,
    /// }
    ///
    /// #[derive(Component, Cleanup)]
    /// struct CleanupMenu;
    ///
    /// #[derive(Component, Cleanup)]
    /// struct CleanupGame;
    ///
    /// App::new()
    ///     .add_state::<AppState>()
    ///     .add_state_cleanup::<_, CleanupMenu>(AppState::Menu)
    ///     .add_state_cleanup::<_, CleanupGame>(AppState::Game);
    /// ```
    fn add_state_cleanup<S: States, C: Cleanup>(&mut self, variant: S) -> &mut Self;
}

impl AddStateCleanup for App {
    fn add_state_cleanup<S: States, C: Cleanup>(&mut self, variant: S) -> &mut Self {
        let cleanup = move |mut commands: Commands, query: Query<Entity, With<C>>| {
            for entity in &query {
                commands.entity(entity).despawn_recursive();
            }
        };

        self.add_systems(OnExit(variant), cleanup)
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{Cleanup, AddStateCleanup};
    use crate as bevy_cleanup;

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

    fn setup_menu(mut commands: Commands) {
        commands.spawn(CleanupMenu);
    }

    fn setup_game(mut commands: Commands) {
        commands.spawn(CleanupGame);
        commands.spawn(CleanupGame);
    }

    fn app() -> App {
        let mut app = App::new();
        app
            .add_state::<AppState>()
            .add_state_cleanup::<_, CleanupMenu>(AppState::Menu)
            .add_state_cleanup::<_, CleanupGame>(AppState::Game)
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnEnter(AppState::Game), setup_game);
        app
    }

    #[test]
    fn remove_on_exit() {
        let mut app = app();
        assert_eq!(0, app.world.entities().len());
        
        app.update();
        assert_eq!(1, app.world.entities().len());

        app.insert_resource(NextState(Some(AppState::Game)));
        app.update();
        assert_eq!(2, app.world.entities().len());

        app.insert_resource(NextState(Some(AppState::Menu)));
        app.update();
        assert_eq!(1, app.world.entities().len());
    }

    #[test]
    fn remove_on_reenter() {
        let mut app = app();
        assert_eq!(0, app.world.entities().len());

        app.update();
        assert_eq!(1, app.world.entities().len());

        app.insert_resource(NextState(Some(AppState::Menu)));
        app.update();
        assert_eq!(1, app.world.entities().len());
    }
}
