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
    #[test]
    fn test() {
        let x = 3;
        println!("{}", x);
    }
}
