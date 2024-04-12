use std::sync::atomic::AtomicBool;

pub struct TeamsStates {
    pub is_muted: AtomicBool,
    pub prev_is_muted: AtomicBool,
    pub is_video_on: AtomicBool,
    pub prev_is_video_on: AtomicBool,
    pub is_hand_raised: AtomicBool,
    pub prev_is_hand_raised: AtomicBool,
    pub is_in_meeting: AtomicBool,
    pub prev_is_in_meeting: AtomicBool,
    pub is_recording_on: AtomicBool,
    pub prev_is_recording_on: AtomicBool,
    pub is_background_blurred: AtomicBool,
    pub prev_is_background_blurred: AtomicBool,
    pub is_sharing: AtomicBool,
    pub prev_is_sharing: AtomicBool,
    pub has_unread_messages: AtomicBool,
    pub prev_has_unread_messages: AtomicBool,
}
