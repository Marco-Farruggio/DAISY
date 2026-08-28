//! Type definitions regarding
//! stream formats
//!
//! Typing in github on mobile
//! is very slow, I will check
//! check this code when I am
//! back home

use std::num::NonZeroU16;

pub enum DataType {
    // Unsigned types
    U8,
    U16,
    U24,
    U32,
    U48,
    U64,

    // Signed types
    I16,
    I24,
    I32,
    I48,
    I64,

    // Floating types
    F32,
    F64,

    // DSD types (no DoP)
    D64,
    D128,
}

type SampleRate = u32;

type ChannelCount = NonZeroU16;
