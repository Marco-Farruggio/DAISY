//! DAISY
//! Duplex Audio Interface Stream API
//!
//! A Rust library for duplex audio
//! on devices with ASIO drivers presenting COM interfaces.
//!
//! This is in no way related to the ASIO SDK by steinberg, and is legally excempt 
//! from its licensing and legal restrictions.
//!
//! All code in this project is under the MIT license, though credit is appreciated.
//!
//! This project was inspired by hearing of JUCE's
//! ability to handle multiple ASIO devices at once.

use crate::devices::Device;
use crate::errors::{DeviceError, StreamInitError};
use crate::format::{DataType, SampleRate, ChannelCount};
