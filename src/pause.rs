use bevy::prelude::*;

pub struct PausePlugin<Set: SystemSet>;

/// Will not compile
impl<Set: SystemSet> Plugin for PausePlugin<Set> {
    fn build(&self, app: &mut App) {
        app.insert_resource(PauseStatus<Set> { paused: false })
            .configure_set(Set.run_if(not(paused::<Set>)));
    }
}

#[derive(Resource)]
pub struct PauseStatus<Set> {
    pub paused: bool,
}

fn paused<Set: SystemSet>(pause_status: Res<PauseStatus<Set>>) -> bool {
    pause_status.paused
}