use std::mem::size_of;
use std::num::from_i32;
use std::marker::MarkerTrait;

#[derive(Debug, Copy, Clone, FromPrimitive, PartialEq, Eq)]
pub enum SampleFormat {
    U8 = 0,
    I16 = 1,
    I32 = 2,
    F32 = 3,
    F64 = 4
}

impl SampleFormat {
    pub fn sample_fmt(&self) -> i32 {
        return *self as i32;
    }

    pub fn size(&self) -> usize {
        match *self {
            SampleFormat::U8 => size_of::<u8>(),
            SampleFormat::I16 => size_of::<i16>(),
            SampleFormat::I32 => size_of::<i32>(),
            SampleFormat::F32 => size_of::<f32>(),
            SampleFormat::F64 => size_of::<f64>(),
        }
    }

    pub fn from_type<T: SampleFormatType>() -> Self {
        <T as SampleFormatType>::sample_fmt()
    }

    pub fn from_sample_fmt(v: i32) -> Option<Self> {
        from_i32(v)
    }
}

pub trait SampleFormatType : MarkerTrait {
    fn sample_fmt() -> SampleFormat;
}

impl SampleFormatType for u8 { // AV_SAMPLE_FMT_U8
    fn sample_fmt() -> SampleFormat { SampleFormat::U8 }
}

impl SampleFormatType for i16 { // AV_SAMPLE_FMT_S16
    fn sample_fmt() -> SampleFormat { SampleFormat::I16 }
}

impl SampleFormatType for i32 { // AV_SAMPLE_FMT_S32
    fn sample_fmt() -> SampleFormat { SampleFormat::I32 }
}

impl SampleFormatType for f32 { // AV_SAMPLE_FMT_FLT
    fn sample_fmt() -> SampleFormat { SampleFormat::F32 }
}

impl SampleFormatType for f64 { // AV_SAMPLE_FMT_DBL
    fn sample_fmt() -> SampleFormat { SampleFormat::F64 }
}
