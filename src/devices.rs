use crate::formats::ChannelCount;
use crate::id::DeviceID;

pub struct Device {
    // placeholder for now...
}

impl Device {
  // todo: these should be results really
    pub fn id(&self) -> Option<DeviceId> {
        None
    }

    pub fn name(&self) -> Option<String> {
        None
    }

    pub fn input_channels(&self) -> Option<ChannelCount> {
        None
    }
}
