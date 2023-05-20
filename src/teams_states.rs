use std::sync::atomic::AtomicBool;

pub struct TeamsStates {
    pub camera_on: AtomicBool,
    pub in_meeting: AtomicBool,
}
