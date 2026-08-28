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

    pub fn output_channels(&self) -> Option<ChannelCount> {
        None
    }

    pub fn channel_names(&self) -> Result<Vec<String>, DeviceError> {
        todo!()
    }

    pub fn channel_name(&self, channel: u16) -> Result<String, DeviceError> {
        todo!()
    }

    pub fn latency(&self) -> Result<u32, DeviceError> {
        todo!()
    }
}
