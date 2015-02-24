// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_double, c_int, c_uint, c_void};
use std::marker::PhantomData;
use std::ffi::CString;
use std::i32;
use std::mem;

use avutil::{ AvDictionary, AvFrame };
use ffi;

pub type AvCodecId = ffi::AVCodecID;

pub const AV_CODEC_ID_H264: AvCodecId = 28;
pub const AV_CODEC_ID_AAC: AvCodecId = 0x15000 + 2;

pub const ERROR_CSTRING: c_int = -1;

pub fn init() {
    unsafe {
        ffi::avcodec_register_all()
    }
}

pub fn version() -> c_uint {
    unsafe {
        ffi::avcodec_version()
    }
}

#[allow(missing_copy_implementations)]
pub struct AvCodec {
    pub codec: *mut ffi::AVCodec,
}

impl AvCodec {
    pub fn find_decoder(codec_id: AvCodecId) -> Result<AvCodec,()> {
        let codec = unsafe {
            ffi::avcodec_find_decoder(codec_id)
        };
        if !codec.is_null() {
            Ok(AvCodec {
                codec: codec,
            })
        } else {
            Err(())
        }
    }
}

pub struct AvCodecContext {
    pub context: ffi::EitherAVCodecContext,
    extra_data: Option<Vec<u8>>,
}

impl AvCodecContext {
    pub fn new(codec: &AvCodec) -> AvCodecContext {
        unsafe {
            let context = ffi::avcodec_alloc_context3(codec.codec);
            AvCodecContext {
                context: ffi::EitherAVCodecContext::from_ptr(context),
                extra_data: None,
            }
        }
    }

    pub fn from_raw(context: *mut ffi::AVCodecContext) -> AvCodecContext {
        AvCodecContext {
            context: ffi::EitherAVCodecContext::from_ptr(context),
            extra_data: None,
        }
    }

    pub fn open(&self, codec: &AvCodec, options: AvDictionary) -> (Result<(),()>, AvDictionary) {
        // The memory management that `libavcodec` expects around the `options` argument is really
        // weird.
        let mut options_not_found = options.dictionary;
        let result;
        unsafe {
            result = ffi::avcodec_open2(self.context.ptr(), codec.codec, &mut options_not_found);
            mem::forget(options);
        }
        let options_not_found = AvDictionary {
            dictionary: options_not_found,
        };
        if result == 0 {
            (Ok(()), options_not_found)
        } else {
            (Err(()), options_not_found)
        }
    }

    pub fn set_extra_data(&mut self, mut extra_data: Vec<u8>) {
        assert!(extra_data.len() <= (i32::MAX as usize));
        unsafe {
            *ffmpeg_ffi_avcodeccontext_field!(self.context, mut extradata) = extra_data.as_mut_slice().as_mut_ptr();
            *ffmpeg_ffi_avcodeccontext_field!(self.context, mut extradata_size) = extra_data.len() as i32;
        }
        self.extra_data = Some(extra_data);
    }

    pub fn set_get_buffer_callback(&mut self, callback: Box<FnMut(&AvFrame)>) {
        unsafe {
            *ffmpeg_ffi_avcodeccontext_field!(self.context, mut opaque) = mem::transmute::<_,*mut c_void>(Box::new(callback));
            *ffmpeg_ffi_avcodeccontext_field!(self.context, mut get_buffer) = get_buffer;
        }
    }

    pub fn decode_video(&self, picture: &AvFrame, packet: &mut AvPacket) -> Result<bool,()> {
        let mut got_picture = 0;
        let result = unsafe {
            ffi::avcodec_decode_video2(self.context.ptr(),
                                       picture.frame,
                                       &mut got_picture,
                                       packet.packet.ptr())
        };
        if result >= 0 && got_picture != 0 {
            Ok(result > 0)
        } else {
            Err(())
        }
    }

    pub fn decode_audio(&self, frame: &AvFrame, packet: &mut AvPacket) -> Result<c_int,()> {
        let mut got_frame = 0;
        let result = unsafe {
            ffi::avcodec_get_frame_defaults(frame.frame);
            ffi::avcodec_decode_audio4(self.context.ptr(),
                                       frame.frame,
                                       &mut got_frame,
                                       packet.packet.ptr())
        };
        if result >= 0 && got_frame != 0 {
            *ffmpeg_ffi_avpacket_field!(packet.packet, mut size) -= result;
            let data = ffmpeg_ffi_avpacket_field!(packet.packet, mut data);
            unsafe {
                *data = data.offset(result as isize);
            }
            Ok(result)
        } else {
            Err(())
        }
    }

    pub fn set_pkt_timebase(&self, timebase: &ffi::AVRational) {
        unsafe {
            ffi::av_codec_set_pkt_timebase(self.context.ptr(), *timebase)
        }
    }

    pub fn get_double_opt(&self, name: &[u8]) -> Result<c_double,c_int> {
        let name = try!(CString::new(name).or_else(|_| Err(ERROR_CSTRING)));
        let mut out_val = 0.0;
        let result = unsafe {
            ffi::av_opt_get_double(self.context.ptr() as *mut c_void,
                                   name.as_ptr(),
                                   0,
                                   &mut out_val)
        };
        if result >= 0 {
            Ok(out_val)
        } else {
            Err(result)
        }
    }

    pub fn get_q_opt(&self, name: &[u8]) -> Result<ffi::AVRational,c_int> {
        let name = try!(CString::new(name).or_else(|_| Err(ERROR_CSTRING)));
        let mut out_val = ffi::AVRational {
            num: 0,
            den: 0,
        };
        let result = unsafe {
            ffi::av_opt_get_q(self.context.ptr() as *mut c_void, name.as_ptr(), 0, &mut out_val)
        };
        if result >= 0 {
            Ok(out_val)
        } else {
            Err(result)
        }
    }

    pub fn sample_rate(&self) -> i32 {
        unsafe {
            *ffmpeg_ffi_avcodeccontext_field!(self.context, sample_rate)
        }
    }

    pub fn channels(&self) -> i32 {
        unsafe {
            *ffmpeg_ffi_avcodeccontext_field!(self.context, channels)
        }
    }
}

extern "C" fn get_buffer(context: *mut ffi::AVCodecContext, frame: *mut ffi::AVFrame) -> c_int {
    let result = unsafe {
        ffi::avcodec_default_get_buffer(context, frame)
    };
    let frame = AvFrame {
        frame: frame,
    };
    unsafe {
        let mut callback = if version() < 0x380d64 {
            let context = mem::transmute::<_,*mut ffi::AVCodecContextV362300>(context);
            mem::transmute::<*mut c_void,Box<Box<FnMut(&AvFrame)>>>((*context).opaque)
        } else {
            let context = mem::transmute::<_,*mut ffi::AVCodecContextV380D64>(context);
            mem::transmute::<*mut c_void,Box<Box<FnMut(&AvFrame)>>>((*context).opaque)
        };
        (*callback)(&frame);
        mem::forget(frame);
    }
    result
}

pub struct AvPacket<'a> {
    pub packet: ffi::EitherAVPacket,
    _marker: PhantomData<&'a AvPacket<'a>>,
}

impl<'a> AvPacket<'a> {
    /// NB: `FF_INPUT_BUFFER_PADDING_SIZE` bytes of data at the end of the slice are ignored!
    pub fn new(data: &'a mut [u8]) -> AvPacket<'a> {
        // Guard against segfaults per the documentation by setting the padding to zero.
        assert!(data.len() <= (i32::MAX as usize));
        assert!(data.len() >= ffi::FF_INPUT_BUFFER_PADDING_SIZE);
        for i in range(data.len() - ffi::FF_INPUT_BUFFER_PADDING_SIZE, data.len()) {
            data[i] = 0
        }

        let mut packet = AvPacket::empty();
        *ffmpeg_ffi_avpacket_field!(packet.packet, mut size) = (data.len() - ffi::FF_INPUT_BUFFER_PADDING_SIZE) as i32;
        *ffmpeg_ffi_avpacket_field!(packet.packet, mut data) = data.as_ptr() as *mut u8;

        packet
    }

    pub fn empty() -> Self {
        unsafe {
            let mut packet = if version() < 0x380d64 {
                ffi::EitherAVPacket::V362300(mem::uninitialized())
            } else {
                ffi::EitherAVPacket::V380D64(mem::uninitialized())
            };
            ffi::av_init_packet(packet.ptr());
            *ffmpeg_ffi_avpacket_field!(packet, mut size) = 0;

            AvPacket {
                packet: packet,
                _marker: PhantomData,
            }
        }
    }

    pub fn has_data(&self) -> bool {
        *ffmpeg_ffi_avpacket_field!(self.packet, size) > 0
    }
}
