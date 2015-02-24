use libc::{ c_int, c_uint, c_void, size_t };
use std::slice::{ from_raw_parts_mut, from_raw_parts };
use std::ptr::null_mut;
use std::mem::forget;
use std::ffi::CString;
use std::old_io::Reader;

use avutil::{ AvDictionary };
use avcodec::{ AvPacket, AvCodec, AvCodecContext };

use ffi;

pub fn init() {
    unsafe {
        ffi::av_register_all()
    }
}

pub fn version() -> c_uint {
    unsafe {
        ffi::avformat_version()
    }
}

pub trait AvioContextCallbacks {
    fn read(&mut self, _data: &mut [u8]) -> Result<usize, ()> { Err(()) }
    fn write(&mut self, _data: &[u8]) -> Result<usize, ()> { Err(()) }
    fn seek(&mut self, _offset: i64, _whence: i32) -> Result<u64, ()> { Err(()) }
}

pub struct AvioContext<C> {
    pub context: *mut ffi::AVIOContext,
    buffer: *mut c_void,
    callbacks: C,
}

fn avio_transform_result<T, R, F: Fn(T) -> R>(r: Result<T, ()>, default: R, f: F) -> R {
    match r {
        Ok(v) => f(v),
        _ => default,
    }
}

impl<C: AvioContextCallbacks> AvioContext<C> {
    unsafe fn cb_userdata<'a>(userdata: *mut c_void) -> &'a mut Self {
        &mut *(userdata as *mut Self)
    }

    extern "C" fn cb_read(userdata: *mut c_void, data: *mut u8, size: c_int) -> c_int {
        unsafe {
            let self_ = AvioContext::<C>::cb_userdata(userdata);
            avio_transform_result(self_.callbacks.read(from_raw_parts_mut(data, size as usize)), -1, |v| v as c_int)
        }
    }

    extern "C" fn cb_write(userdata: *mut c_void, data: *mut u8, size: c_int) -> c_int {
        unsafe {
            let self_ = AvioContext::<C>::cb_userdata(userdata);
            avio_transform_result(self_.callbacks.write(from_raw_parts(data, size as usize)), -1, |v| v as c_int)
        }
    }

    extern "C" fn cb_seek(userdata: *mut c_void, position: i64, whence: c_int) -> i64 {
        unsafe {
            let self_ = AvioContext::<C>::cb_userdata(userdata);
            avio_transform_result(self_.callbacks.seek(position, whence), -1, |v| v as i64)
        }
    }

    pub fn new(buffer_size: usize, writable: bool, callbacks: C) -> Box<Self> {
        unsafe {
            let mut self_ = Box::new(AvioContext {
                context: null_mut(),
                buffer: ffi::av_malloc(buffer_size as size_t),
                callbacks: callbacks,
            });

            self_.context = ffi::avio_alloc_context(self_.buffer as *mut _,
                                                    buffer_size as c_int,
                                                    if writable { 1 } else { 0 },
                                                    &mut *self_ as *mut _ as *mut c_void,
                                                    Some(AvioContext::<C>::cb_read),
                                                    Some(AvioContext::<C>::cb_write),
                                                    Some(AvioContext::<C>::cb_seek));

            self_
        }
    }
}

pub struct AvioContextReader<R> {
    reader: R,
}

impl<R> AvioContextReader<R> {
    pub fn new(reader: R) -> Self {
        AvioContextReader {
            reader: reader,
        }
    }
}

impl<R: Reader> AvioContextCallbacks for AvioContextReader<R> {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, ()> {
        self.reader.read(data).or_else(|_| Err(()))
    }
}

pub struct AvFormatContext<T = ()> {
    pub context: *mut ffi::AVFormatContext,
    _avio: Option<Box<T>>,
}

impl<T> AvFormatContext<AvioContext<T>> {
    pub fn open_input(avio: Box<AvioContext<T>>, options: AvDictionary) -> (Result<Self, c_int>, AvDictionary) {
        unsafe {
            let context = ffi::avformat_alloc_context();
            (*context).flags |= ffi::AVFMT_FLAG_CUSTOM_IO;
            (*context).pb = avio.context;
            AvFormatContext::open_internal(context, Some(avio), "", options)
        }
    }
}


impl<T> AvFormatContext<T> {
    pub fn open_file(filename: &str, options: AvDictionary) -> (Result<Self, c_int>, AvDictionary) {
        unsafe {
            AvFormatContext::open_internal(ffi::avformat_alloc_context(), None, filename, options)
        }
    }

    fn open_internal(context: *mut ffi::AVFormatContext, avio: Option<Box<T>>, filename: &str, options: AvDictionary) -> (Result<Self, c_int>, AvDictionary) {
        unsafe {
            let mut context = context;
            let mut options_not_found = options.dictionary;
            let result = ffi::avformat_open_input(&mut context, CString::new(filename).unwrap().as_ptr(), null_mut(), &mut options_not_found);

            forget(options);
            let options_not_found = AvDictionary {
                dictionary: options_not_found,
            };

            if result == 0 {
                (Ok(AvFormatContext {
                    context: context,
                    _avio: avio,
                }), options_not_found)
            } else {
                (Err(result), options_not_found)
            }
        }
    }

    pub fn find_stream_info(&self, options: AvDictionary) -> (Result<(), c_int>, AvDictionary) {
        unsafe {
            let mut options_not_found = options.dictionary;
            let result = ffi::avformat_find_stream_info(self.context, &mut options_not_found);

            forget(options);
            let options_not_found = AvDictionary {
                dictionary: options_not_found,
            };

            if result == 0 {
                (Ok(()), options_not_found)
            } else {
                (Err(result), options_not_found)
            }
        }
    }

    pub fn find_stream(&self, kind: c_int) -> Option<usize> {
        unsafe {
            let context = &*self.context;
            for i in 0..context.nb_streams {
                let stream = *context.streams.offset(i as isize);
                if !stream.is_null() {
                    let codec = (*stream).codec;
                    if !codec.is_null() {
                        let codec = ffi::EitherAVCodecContext::from_ptr(codec);
                        if *ffmpeg_ffi_avcodeccontext_field!(codec, codec_type) == kind {
                            return Some(i as usize)
                        }
                    }
                }
            }

            None
        }
    }

    pub fn open_stream(&self, index: usize, options: AvDictionary) -> (Result<AvCodecContext, c_int>, AvDictionary) {
        unsafe {
            let context = &*self.context;
            assert!((index as c_uint) < context.nb_streams);

            let stream = *context.streams.offset(index as isize);
            if stream.is_null() { return (Err(-1), options) }
            let stream = &*stream;

            let codec = stream.codec;
            if codec.is_null() { return (Err(-1), options) }
            let codec = ffi::EitherAVCodecContext::from_ptr(codec);

            let decoder = if let Ok(decoder) = AvCodec::find_decoder(*ffmpeg_ffi_avcodeccontext_field!(codec, codec_id)) {
                decoder
            } else {
                return (Err(-1), options)
            };

            let codec = AvCodecContext::from_raw(codec.ptr());
            let (r, o) = codec.open(&decoder, options);
            (r.map(|_| codec).or_else(|_| Err(-1)), o)
        }
    }

    pub fn read_packet(&self, packet: &mut AvPacket) -> Result<(), c_int> {
        unsafe {
            let result = ffi::av_read_frame(self.context, packet.packet.ptr());
            if result == 0 {
                Ok(())
            } else {
                Err(result)
            }
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for AvFormatContext<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::avformat_close_input(&mut self.context);
        }
    }
}
