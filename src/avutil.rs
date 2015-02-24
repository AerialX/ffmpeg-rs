// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_int, c_void};
use std::any::Any;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;
use ffi;

pub struct AvFrame {
    pub frame: *mut ffi::AVFrame,
}

impl Drop for AvFrame {
    fn drop(&mut self) {
        unsafe {
            ffi::av_frame_free(&mut self.frame)
        }
    }
}

impl AvFrame {
    pub fn new() -> AvFrame {
        unsafe {
            let frame = AvFrame {
                frame: ffi::av_frame_alloc(),
            };
            (*frame.frame).opaque = ptr::null_mut();
            frame
        }
    }

    pub fn width(&self) -> c_int {
        unsafe {
            (*self.frame).width
        }
    }

    pub fn height(&self) -> c_int {
        unsafe {
            (*self.frame).height
        }
    }

    pub fn linesize(&self, plane_index: usize) -> c_int {
        unsafe {
            (*self.frame).linesize[plane_index]
        }
    }

    pub fn sample_count(&self) -> c_int {
        unsafe {
            (*self.frame).nb_samples
        }
    }

    pub fn format(&self) -> c_int {
        unsafe {
            (*self.frame).format
        }
    }

    pub fn user_data<'a>(&'a self) -> &'a Any {
        unsafe {
            assert!(!(*self.frame).opaque.is_null());
            let user_data = mem::transmute::<_,&Box<Box<Any>>>(&(*self.frame).opaque);
            &***user_data
        }
    }

    pub fn set_user_data(&self, user_data: Box<Any>) {
        unsafe {
            if !(*self.frame).opaque.is_null() {
                drop(mem::transmute::<_,Box<Box<Any>>>((*self.frame).opaque));
            }
            (*self.frame).opaque = mem::transmute::<Box<Box<Any>>,*mut c_void>(Box::new(user_data))
        }
    }

    pub fn pts(&self) -> i64 {
        unsafe {
            (*self.frame).pts
        }
    }

    pub fn pkt_pts(&self) -> i64 {
        unsafe {
            (*self.frame).pkt_pts
        }
    }

    pub fn pkt_dts(&self) -> i64 {
        unsafe {
            (*self.frame).pkt_dts
        }
    }

    pub fn video_data<'a>(&'a self, plane_index: usize) -> &'a [u8] {
        let len = self.linesize(plane_index) * self.height();
        unsafe {
            slice::from_raw_parts_mut((*self.frame).data[plane_index], len as usize)
        }
    }

    pub fn audio_data<'a>(&'a self, channel: usize, channels: i32) -> &'a [u8] {
        let len = samples::buffer_size(channels,
                                       self.sample_count(),
                                       self.format(),
                                       true).unwrap()
                                            .linesize;
        unsafe {
            slice::from_raw_parts_mut((*self.frame).data[channel], len as usize)
        }
    }
}

pub struct AvDictionary {
    pub dictionary: *mut ffi::AVDictionary,
}

impl Drop for AvDictionary {
    fn drop(&mut self) {
        unsafe {
            ffi::av_dict_free(&mut self.dictionary)
        }
    }
}

impl AvDictionary {
    pub fn new() -> AvDictionary {
        AvDictionary {
            dictionary: ptr::null_mut(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            let key = CString::new(key.as_bytes()).unwrap();
            let value = CString::new(value.as_bytes()).unwrap();
            assert!(ffi::av_dict_set(&mut self.dictionary, key.as_ptr(), value.as_ptr(), 0) >= 0);
        }
    }
}

pub mod samples {
    use libc::c_int;
    use ffi;

    #[derive(Copy)]
    pub struct BufferSizeResult {
        pub buffer_size: c_int,
        pub linesize: c_int,
    }

    pub fn buffer_size(channels: c_int, samples: c_int, format: ffi::AVSampleFormat, align: bool)
                       -> Result<BufferSizeResult,c_int> {
        let mut linesize = 0;
        let align = if !align {
            0
        } else {
            1
        };
        let result = unsafe {
            ffi::av_samples_get_buffer_size(&mut linesize, channels, samples, format, align)
        };
        if result >= 0 {
            Ok(BufferSizeResult {
                buffer_size: result,
                linesize: linesize,
            })
        } else {
            Err(result)
        }
    }
}
