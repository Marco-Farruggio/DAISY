//! API signature
//!
//! (Error handling is simplified
//! for the sake of example)
use daisy::{
    Device,
    devices,
    Stream,
    DeviceError,
    StreamInitError,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devs = devices()?;
    let dev = devs.first()?;

    let peaks = Arc::new([AtomicF32::new(0.0); 16]);
    let stream = dev.stream(
        0..16,
        0..8,
        move |inputs, outputs| {
            for (i, channel) in inputs.iter().enumerate() {
                peaks[i].set(channel.max());
            }
            for channel in outputs {
                channel.fill(0.0);
            }
        }
    })?;
}
