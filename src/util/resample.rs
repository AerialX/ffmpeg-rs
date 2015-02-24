use util::SampleFormatType;
use libc::{ c_int };
use std::marker::PhantomData;
use std::default::Default;
use ffi;

pub struct Resample<I = i16, O = i16> {
    context: *mut ffi::ReSampleContext,
    out_channels: usize,
    out_sample_rate: usize,
    in_channels: usize,
    in_sample_rate: usize,
    _marker: PhantomData<(I, O)>,
}

impl<I: SampleFormatType, O: SampleFormatType + Clone + Default> Resample<I, O> {
    pub fn new(in_channels: usize, out_channels: usize, in_sample_rate: usize, out_sample_rate: usize) -> Self {
        unsafe {
            let context = ffi::av_audio_resample_init(out_channels as c_int,
                                                      in_channels as c_int,
                                                      out_sample_rate as c_int,
                                                      in_sample_rate as c_int,
                                                      <O as SampleFormatType>::sample_fmt().sample_fmt(),
                                                      <I as SampleFormatType>::sample_fmt().sample_fmt(),
                                                      16, 10, 0, 0.8);

            Resample {
                out_channels: out_channels,
                out_sample_rate: out_sample_rate,
                in_channels: in_channels,
                in_sample_rate: in_sample_rate,
                context: context,
                _marker: PhantomData,
            }
        }
    }

    pub fn resample_data(&self, i: &[I], o: &mut [O]) -> Result<(usize, usize), c_int> {
        unsafe {
            let il = i.len() / self.in_channels;
            let result = ffi::audio_resample(self.context,
                                             o.as_ptr() as *mut O as *mut i16,
                                             i.as_ptr() as *mut u8 as *mut i16,
                                             il as c_int);

            if result >= 0 {
                Ok((il as usize, result as usize))
            } else {
                Err(result)
            }
        }
    }

    pub fn resample(&self, i: &[I], o: &mut Vec<O>) -> Result<usize, c_int> {
        let il = i.len() / self.in_channels;
        let ol = il * self.out_sample_rate / self.in_sample_rate + 16;
        o.resize(ol * self.out_channels, <O as Default>::default());
        let (consumed, ol) = try!(self.resample_data(i, o));
        o.truncate(ol * self.out_channels);
        Ok(consumed)
    }
}

#[unsafe_destructor]
impl<I, O> Drop for Resample<I, O> {
    fn drop(&mut self) {
        unsafe {
            ffi::audio_resample_close(self.context)
        }
    }
}
