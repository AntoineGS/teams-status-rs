use crate::teams_ws::states::TeamsStates;
use async_trait::async_trait;

pub trait StopController {}

#[async_trait]
pub trait Listener {
    async fn notify_changed(
        &mut self,
        teams_states: &TeamsStates,
        force_update: bool,
    ) -> anyhow::Result<()>;
    fn reconnect(&mut self);
}
