use std::sync::atomic::AtomicBool;

pub struct TeamsStates {
    pub is_video_on: AtomicBool,
    pub is_in_meeting: AtomicBool,
}
