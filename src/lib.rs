use bevy::prelude::*;

#[cfg(feature = "derive")]
pub use bevy_cleanup_derive::Cleanup;

pub trait Cleanup: Component {}

pub trait AddStateCleanup {
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
