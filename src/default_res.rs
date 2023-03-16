



use crate::*;

#[derive(Resource, Default, Debug)]
pub struct Time{
    pub delta_time: f32,
    pub delta_time_millis: u128,
    pub delta_time_micros: u128,
    pub delta_time_nanos: u128,
}


#[derive(Resource, Default, Debug)]
pub struct EngineConfig{
    pub(crate) limit_fps: Option<u32>, // None - no limit; Option<Limited_Value>
    pub(crate) fullscreen: bool,
}

impl EngineConfig{
    pub fn limit_fps(value: u32){
        let mut slck = EngineConfig::write();
        if value == 0{
            slck.limit_fps = None;
        } else {
            slck.limit_fps = Some(value);
        }
    }
}
