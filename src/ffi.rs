// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub const FF_INPUT_BUFFER_PADDING_SIZE: usize = 32;

use libc::{c_char, c_double, c_float, c_int, c_short, c_uint, c_void, size_t };

pub type AVCodecID = c_int;
pub type AVColorRange = c_int;
pub type AVColorSpace = c_int;
pub type AVPictureType = c_int;
pub type AVStreamParseType = c_int;
pub type AVSampleFormat = c_int;
pub type AVDurationEstimationMethod = c_int;
pub type AVDiscard = c_int;

pub const AV_NUM_DATA_POINTERS: usize = 8;

pub const AVFMT_FLAG_GENPTS: c_int          = 0x00001;
pub const AVFMT_FLAG_IGNIDX: c_int          = 0x00002;
pub const AVFMT_FLAG_NONBLOCK: c_int        = 0x00004;
pub const AVFMT_FLAG_IGNDTS: c_int          = 0x00008;
pub const AVFMT_FLAG_NOFILLIN: c_int        = 0x00010;
pub const AVFMT_FLAG_NOBUFFER: c_int        = 0x00040;
pub const AVFMT_FLAG_CUSTOM_IO: c_int       = 0x00080;
pub const AVFMT_FLAG_DISCARD_CORRUPT: c_int = 0x00100;
pub const AVFMT_FLAG_FLUSH_PACKETS: c_int   = 0x00200;
pub const AVFMT_FLAG_BITEXACT: c_int        = 0x00400;
pub const AVFMT_FLAG_MP4A_LATM: c_int       = 0x08000;
pub const AVFMT_FLAG_SORT_DTS: c_int        = 0x10000;
pub const AVFMT_FLAG_PRIV_OPT: c_int        = 0x20000;
pub const AVFMT_FLAG_KEEP_SIDE_DATA: c_int  = 0x40000;

pub const AVMEDIA_TYPE_VIDEO: c_int = 0;
pub const AVMEDIA_TYPE_AUDIO: c_int = 1;
pub const AVMEDIA_TYPE_DATA: c_int = 2;
pub const AVMEDIA_TYPE_SUBTITLE: c_int = 3;
pub const AVMEDIA_TYPE_ATTACHMENT: c_int = 4;

#[repr(C)]
pub struct AVBuffer;
#[repr(C)]
pub struct AVClass;
#[repr(C)]
pub struct AVCodec;
#[repr(C)]
pub struct AVCodecContext;
#[repr(C)]
pub struct AVCodecInternal;
#[repr(C)]
pub struct AVDictionary;
#[repr(C)]
pub struct AVFrameSideData;
#[repr(C)]
pub struct AVPacket;
#[repr(C)]
pub struct AVPacketSideData;
#[repr(C)]
pub struct AVPanScan;
#[repr(C)]
pub struct AVIOContext;
#[repr(C)]
pub struct AVInputFormat;
#[repr(C)]
pub struct AVOutputFormat;
#[repr(C)]
pub struct AVProgram;
#[repr(C)]
pub struct AVPacketList;
#[repr(C)]
pub struct AVFormatInternal;
#[repr(C)]
pub struct AVChapter;
#[repr(C)]
pub struct AVCodecParserContext;
#[repr(C)]
pub struct AVIndexEntry;
#[repr(C)]
pub struct ReSampleContext;

#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut u8,
    pub size: c_int,
}

/// `AVPacket` for `libavcodec` below version 0x380D64.
#[repr(C)]
pub struct AVCodecContextV362300 {
    pub av_class: *const AVClass,
    pub log_level_offset: c_int,
    pub codec_type: c_int,
    pub codec: *const AVCodec,
    pub codec_name: [c_char; 32],
    pub codec_id: AVCodecID,
    pub codec_tag: c_uint,
    pub stream_codec_tag: c_uint,
    pub sub_id: c_int,
    pub priv_data: *mut c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut c_void,
    pub bit_rate: c_int,
    pub bit_rate_tolerance: c_int,
    pub global_quality: c_int,
    pub compression_level: c_int,
    pub flags: c_int,
    pub flags2: c_int,
    pub extradata: *mut u8,
    pub extradata_size: c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: c_int,
    pub delay: c_int,
    pub width: c_int,
    pub height: c_int,
    pub coded_width: c_int,
    pub coded_height: c_int,
    pub gop_size: c_int,
    pub pix_fmt: c_int,
    pub me_method: c_int,
    pub draw_horiz_band: extern "C" fn(s: *mut AVCodecContext,
                                       src: *const AVFrame,
                                       offset: [c_int; AV_NUM_DATA_POINTERS],
                                       y: c_int,
                                       band_type: c_int,
                                       height: c_int),
    pub get_format: extern "C" fn(s: *mut AVCodecContext, fmt: *const c_int),
    pub max_b_frames: c_int,
    pub b_quant_factor: c_float,
    pub rc_strategy: c_int,
    pub b_frame_strategy: c_int,
    pub luma_elim_threshold: c_int,
    pub chroma_elim_threshold: c_int,
    pub b_quant_offset: c_float,
    pub has_b_frames: c_int,
    pub mpeg_quant: c_int,
    pub i_quant_factor: c_float,
    pub i_quant_offset: c_float,
    pub lumi_masking: c_float,
    pub temporal_cplx_masking: c_float,
    pub spatial_cplx_masking: c_float,
    pub p_masking: c_float,
    pub dark_masking: c_float,
    pub slice_count: c_int,
    pub prediction_method: c_int,
    pub slice_offset: *mut c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: c_int,
    pub me_sub_cmp: c_int,
    pub mb_cmp: c_int,
    pub ildct_cmp: c_int,
    pub dia_size: c_int,
    pub last_predictor_count: c_int,
    pub pre_me: c_int,
    pub me_pre_cmp: c_int,
    pub pre_dia_size: c_int,
    pub me_subpel_quality: c_int,
    pub dtg_active_format: c_int,
    pub me_range: c_int,
    pub intra_quant_bias: c_int,
    pub inter_quant_bias: c_int,
    pub color_table_id: c_int,
    pub slice_flags: c_int,
    pub xvmc_acceleration: c_int,   // NB: Behind `#ifdef FF_API_XVMC`!
    pub mb_decision: c_int,
    pub intra_matrix: *mut u16,
    pub inter_matrix: *mut u16,
    pub scenechange_threshold: c_int,
    pub noise_reduction: c_int,
    pub inter_threshold: c_int,
    pub quantizer_noise_shaping: c_int,
    pub me_threshold: c_int,
    pub mb_threshold: c_int,
    pub intra_dc_precision: c_int,
    pub skip_top: c_int,
    pub skip_bottom: c_int,
    pub border_masking: c_float,
    pub mb_lmin: c_int,
    pub mb_lmax: c_int,
    pub me_penalty_compensation: c_int,
    pub bidir_refine: c_int,
    pub brd_scale: c_int,
    pub keyint_min: c_int,
    pub refs: c_int,
    pub chromaoffset: c_int,
    pub scenechange_factor: c_int,
    pub mv0_threshold: c_int,
    pub b_sensitivity: c_int,
    pub color_primaries: c_int,
    pub color_trc: c_int,
    pub colorspace: c_int,
    pub color_range: c_int,
    pub chroma_sample_location: c_int,
    pub slices: c_int,
    pub field_order: c_int,
    pub sample_rate: c_int,
    pub channels: c_int,
    pub sample_fmt: c_int,
    pub frame_size: c_int,
    pub frame_number: c_int,
    pub block_align: c_int,
    pub cutoff: c_int,
    pub request_channels: c_int,    // NB: Behind `#ifdef FF_API_REQUEST_CHANNELS`!
    pub channel_layout: u64,
    pub request_channel_layout: u64,
    pub audio_service_type: c_int,
    pub request_sample_fmt: c_int,
    // NB: The next three are behind `#ifdef FF_API_GET_BUFFER`!
    pub get_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame) -> c_int,
    pub release_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame),
    pub reget_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame),
    pub get_buffer2: extern "C" fn(s: *mut AVCodecContext, frame: *mut AVFrame, flags: c_int)
                                   -> c_int,
    // More follow...
}

/// `AVPacket` for `libavcodec` version 0x380D64 or greater.
#[repr(C)]
pub struct AVCodecContextV380D64 {
    pub av_class: *const AVClass,
    pub log_level_offset: c_int,
    pub codec_type: c_int,
    pub codec: *const AVCodec,
    pub codec_name: [c_char; 32],
    pub codec_id: AVCodecID,
    pub codec_tag: c_uint,
    pub stream_codec_tag: c_uint,
    pub priv_data: *mut c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut c_void,
    pub bit_rate: c_int,
    pub bit_rate_tolerance: c_int,
    pub global_quality: c_int,
    pub compression_level: c_int,
    pub flags: c_int,
    pub flags2: c_int,
    pub extradata: *mut u8,
    pub extradata_size: c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: c_int,
    pub delay: c_int,
    pub width: c_int,
    pub height: c_int,
    pub coded_width: c_int,
    pub coded_height: c_int,
    pub gop_size: c_int,
    pub pix_fmt: c_int,
    pub me_method: c_int,
    pub draw_horiz_band: extern "C" fn(s: *mut AVCodecContext,
                                       src: *const AVFrame,
                                       offset: [c_int; AV_NUM_DATA_POINTERS],
                                       y: c_int,
                                       band_type: c_int,
                                       height: c_int),
    pub get_format: extern "C" fn(s: *mut AVCodecContext, fmt: *const c_int),
    pub max_b_frames: c_int,
    pub b_quant_factor: c_float,
    pub rc_strategy: c_int,
    pub b_frame_strategy: c_int,
    pub b_quant_offset: c_float,
    pub has_b_frames: c_int,
    pub mpeg_quant: c_int,
    pub i_quant_factor: c_float,
    pub i_quant_offset: c_float,
    pub lumi_masking: c_float,
    pub temporal_cplx_masking: c_float,
    pub spatial_cplx_masking: c_float,
    pub p_masking: c_float,
    pub dark_masking: c_float,
    pub slice_count: c_int,
    pub prediction_method: c_int,
    pub slice_offset: *mut c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: c_int,
    pub me_sub_cmp: c_int,
    pub mb_cmp: c_int,
    pub ildct_cmp: c_int,
    pub dia_size: c_int,
    pub last_predictor_count: c_int,
    pub pre_me: c_int,
    pub me_pre_cmp: c_int,
    pub pre_dia_size: c_int,
    pub me_subpel_quality: c_int,
    pub dtg_active_format: c_int,
    pub me_range: c_int,
    pub intra_quant_bias: c_int,
    pub inter_quant_bias: c_int,
    pub slice_flags: c_int,
    pub xvmc_acceleration: c_int,   // NB: Behind `#ifdef FF_API_XVMC`!
    pub mb_decision: c_int,
    pub intra_matrix: *mut u16,
    pub inter_matrix: *mut u16,
    pub scenechange_threshold: c_int,
    pub noise_reduction: c_int,
    pub me_threshold: c_int,
    pub mb_threshold: c_int,
    pub intra_dc_precision: c_int,
    pub skip_top: c_int,
    pub skip_bottom: c_int,
    pub border_masking: c_float,
    pub mb_lmin: c_int,
    pub mb_lmax: c_int,
    pub me_penalty_compensation: c_int,
    pub bidir_refine: c_int,
    pub brd_scale: c_int,
    pub keyint_min: c_int,
    pub refs: c_int,
    pub chromaoffset: c_int,
    pub scenechange_factor: c_int,
    pub mv0_threshold: c_int,
    pub b_sensitivity: c_int,
    pub color_primaries: c_int,
    pub color_trc: c_int,
    pub colorspace: c_int,
    pub color_range: c_int,
    pub chroma_sample_location: c_int,
    pub slices: c_int,
    pub field_order: c_int,
    pub sample_rate: c_int,
    pub channels: c_int,
    pub sample_fmt: c_int,
    pub frame_size: c_int,
    pub frame_number: c_int,
    pub block_align: c_int,
    pub cutoff: c_int,
    pub request_channels: c_int,    // NB: Behind `#ifdef FF_API_REQUEST_CHANNELS`!
    pub channel_layout: u64,
    pub request_channel_layout: u64,
    pub audio_service_type: c_int,
    pub request_sample_fmt: c_int,
    // NB: The next three are behind `#ifdef FF_API_GET_BUFFER`!
    pub get_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame) -> c_int,
    pub release_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame),
    pub reget_buffer: extern "C" fn(c: *mut AVCodecContext, pic: *mut AVFrame),
    pub get_buffer2: extern "C" fn(s: *mut AVCodecContext, frame: *mut AVFrame, flags: c_int)
                                   -> c_int,
    // More follow...
}

pub enum EitherAVCodecContext {
    V362300(*mut AVCodecContextV362300),
    V380D64(*mut AVCodecContextV380D64),
}

impl EitherAVCodecContext {
    pub fn ptr(&self) -> *mut AVCodecContext {
        match *self {
            EitherAVCodecContext::V362300(context) => context as *mut AVCodecContext,
            EitherAVCodecContext::V380D64(context) => context as *mut AVCodecContext,
        }
    }

    pub fn from_ptr(context: *mut AVCodecContext) -> Self {
        unsafe {
            if avcodec_version() < 0x380d64 {
                EitherAVCodecContext::V362300(context as *mut AVCodecContextV362300)
            } else {
                EitherAVCodecContext::V380D64(context as *mut AVCodecContextV380D64)
            }
        }
    }
}

#[macro_export]
macro_rules! ffmpeg_ffi_avcodeccontext_field {
    ($ctx: expr, mut $prop: ident) => (
        match &$ctx {
            &$crate::ffi::EitherAVCodecContext::V362300(v) => &mut (&mut *v).$prop,
            &$crate::ffi::EitherAVCodecContext::V380D64(v) => &mut (&mut *v).$prop,
        }
    );
    ($ctx: expr, $prop: ident) => (
        match &$ctx {
            &$crate::ffi::EitherAVCodecContext::V362300(v) => &(&*v).$prop,
            &$crate::ffi::EitherAVCodecContext::V380D64(v) => &(&*v).$prop,
        }
    );
}

#[macro_export]
macro_rules! ffmpeg_ffi_avpacket_field {
    ($ctx: expr, mut $prop: ident) => (
        match &mut $ctx {
            &mut $crate::ffi::EitherAVPacket::V362300(ref mut v) => &mut v.$prop,
            &mut $crate::ffi::EitherAVPacket::V380D64(ref mut v) => &mut v.$prop,
        }
    );
    ($ctx: expr, $prop: ident) => (
        match &$ctx {
            &$crate::ffi::EitherAVPacket::V362300(ref v) => &v.$prop,
            &$crate::ffi::EitherAVPacket::V380D64(ref v) => &v.$prop,
        }
    );
}

#[repr(C)]
pub struct AVFrame {
    pub data: [*mut u8; AV_NUM_DATA_POINTERS],
    pub linesize: [c_int; AV_NUM_DATA_POINTERS],
    pub extended_data: *mut *mut u8,
    pub width: c_int,
    pub height: c_int,
    pub nb_samples: c_int,
    pub format: c_int,
    pub keyframe: c_int,
    pub pict_type: AVPictureType,
    pub base: [*mut u8; AV_NUM_DATA_POINTERS],
    pub sample_aspect_ratio: AVRational,
    pub pts: i64,
    pub pkt_pts: i64,
    pub pkt_dts: i64,
    pub coded_picture_number: c_int,
    pub display_picture_number: c_int,
    pub quality: c_int,
    pub reference: c_int,
    pub qscale_table: *mut i8,
    pub qstride: c_int,
    pub qscale_type: c_int,
    pub mbskip_table: *mut u8,
    pub motion_val: [[*mut i16; 2]; 2],
    pub mb_type: *mut u32,
    pub dct_coeff: *mut c_short,
    pub ref_index: [*mut i8; 2],
    pub opaque: *mut c_void,
    pub error: [u64; AV_NUM_DATA_POINTERS],
    pub frame_type: c_int,
    pub repeat_pict: c_int,
    pub interlaced_frame: c_int,
    pub top_field_first: c_int,
    pub palette_has_changed: c_int,
    pub buffer_hints: c_int,
    pub pan_scan: *mut AVPanScan,
    pub reordered_opaque: i64,
    pub hwaccel_picture_private: *mut c_void,
    pub owner: *mut AVCodecContext,
    pub thread_opaque: *mut c_void,
    pub motion_subsample_log2: u8,
    pub sample_rate: c_int,
    pub channel_layout: u64,
    pub buf: [*mut AVBufferRef; AV_NUM_DATA_POINTERS],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: c_int,
    pub flags: c_int,
    pub best_effort_timestamp: i64,
    pub pkt_pos: i64,
    pub pkt_duration: i64,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: c_int,
    pub channels: c_int,
    pub pkt_size: c_int,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub qp_table_buf: *mut AVBufferRef,
}

/// `AVPacket` for `libavcodec` below version 0x380D64.
#[repr(C)]
pub struct AVPacketV362300 {
    pub pts: i64,
    pub dts: i64,
    pub data: *mut u8,
    pub size: c_int,
    pub stream_index: c_int,
    pub flags: c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: c_int,
    pub duration: c_int,
    pub destruct: extern "C" fn(packet: *mut AVPacket),
    pub private: *mut c_void,
    pub pos: i64,
    pub convergence_duration: i64,
}

/// `AVPacket` for `libavcodec` version 0x380d64 and up.
#[repr(C)]
pub struct AVPacketV380D64 {
    pub buf: *mut AVBufferRef,
    pub pts: i64,
    pub dts: i64,
    pub data: *mut u8,
    pub size: c_int,
    pub stream_index: c_int,
    pub flags: c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: c_int,
    pub duration: c_int,
    pub destruct: extern "C" fn(packet: *mut AVPacket),
    pub private: *mut c_void,
    pub pos: i64,
    pub convergence_duration: i64,
}

pub enum EitherAVPacket {
    V362300(AVPacketV362300),
    V380D64(AVPacketV380D64),
}

impl EitherAVPacket {
    pub fn ptr(&mut self) -> *mut AVPacket {
        match *self {
            EitherAVPacket::V362300(ref mut packet) => {
                packet as *mut AVPacketV362300 as *mut AVPacket
            }
            EitherAVPacket::V380D64(ref mut packet) => {
                packet as *mut AVPacketV380D64 as *mut AVPacket
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
pub struct AVRational {
    pub num: c_int,
    pub den: c_int,
}

pub type av_format_control_message = extern "C" fn(s: *mut AVFormatContext,
                                                   type_: c_int,
                                                   data: *mut c_void,
                                                   data_size: size_t);

#[repr(C)]
pub struct AVIOInterruptCB {
    callback: extern "C" fn(*mut c_void) -> c_int,
    opaque: *mut c_void,
}

#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: c_int,
    pub nb_streams: c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [c_char; 1024],
    pub start_time: i64,
    pub duration: i64,
    pub bit_rate: c_int,
    pub packet_size: c_uint,
    pub max_delay: c_int,
    pub flags: c_int,
    pub probesize: c_uint,
    pub max_analyze_duration: c_int,
    pub key: *const u8,
    pub keylen: c_int,
    pub nb_programs: c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: c_uint,
    pub max_picture_buffer: c_uint,
    pub nb_chapters: c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: i64,
    pub fps_probe_size: c_int,
    pub error_recognition: c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: c_int,
    pub max_interleave_delta: i64,
    pub strict_std_compliance: c_int,
    pub event_flags: c_int,
    pub max_ts_probe: c_int,
    pub avoid_negative_ts: c_int,
    pub ts_id: c_int,
    pub audio_preload: c_int,
    pub max_chunk_duration: c_int,
    pub max_chunk_size: c_int,
    pub use_wallclock_as_timestamps: c_int,
    pub avio_flags: c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: i64,
    pub correct_ts_overflow: c_uint,
    pub seek2any: c_int,
    pub flush_packets: c_int,
    pub probe_score: c_int,
    pub format_probesize: c_int,
    pub codec_whitelist: *mut c_char,
    pub format_whitelist: *mut c_char,
    pub packet_buffer: *mut AVPacketList,
    pub packet_buffer_end: *mut AVPacketList,
    pub data_offset: i64,
    pub raw_packet_buffer: *mut AVPacketList,
    pub raw_packet_buffer_end: *mut AVPacketList,
    pub parse_queue: *mut AVPacketList,
    pub parse_queue_end: *mut AVPacketList,
    pub raw_packet_buffer_remaining_size: c_int,
    pub offset: i64,
    pub offset_timebase: AVRational,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub metadata_header_padding: c_int,
    pub opaque: *mut c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: i64,
    pub max_analyze_duration2: i64,
    pub probesize2: i64,
    pub dump_separator: *mut u8,
}

#[repr(C)]
pub struct AVFrac {
    val: i64,
    num: i64,
    den: i64,
}

#[repr(C)]
pub struct AVProbeData {
    filename: *const c_char,
    buf: *mut u8,
    buf_size: c_int,
    mime_type: *const c_char,
}

#[repr(C)]
pub struct AVStream_info {
    pub last_dts: i64,
    pub duration_gcd: i64,
    pub duration_count: c_int,
    pub rfps_duration_sum: i64,
    pub duration_error: *mut c_void,
    pub codec_info_duration: i64,
    pub codec_info_duration_fields: i64,
    pub found_decoder: c_int,
    pub last_duration: i64,
    pub fps_first_dts: i64,
    pub fps_first_dts_idx: c_int,
    pub fps_last_dts: i64,
    pub fps_last_dts_idx: c_int,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct AVStream {
    pub index: c_int,
    pub id: c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut c_void,
    pub pts: AVFrac,
    pub time_base: AVRational,
    pub start_time: i64,
    pub duration: i64,
    pub nb_frames: i64,
    pub disposition: c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: c_int,
    pub event_flags: c_int,
    pub info: *mut AVStream_info,
    pub pts_wrap_bits: c_int,
    pub first_dts: i64,
    pub cur_dts: i64,
    pub last_IP_pts: i64,
    pub last_IP_duration: c_int,
    pub probe_packets: c_int,
    pub codec_info_nb_frames: c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [i64; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: c_int,
    pub index_entries_allocated_size: c_uint,
    pub r_frame_rate: AVRational,
    pub stream_identifier: c_int,
    pub interleaver_chunk_size: i64,
    pub interleaver_chunk_duration: i64,
    pub request_probe: c_int,
    pub skip_to_keyframe: c_int,
    pub skip_samples: c_int,
    pub first_discard_sample: i64,
    pub last_discard_sample: i64,
    pub nb_decoded_frames: c_int,
    pub mux_ts_offset: i64,
    pub pts_wrap_reference: i64,
    pub pts_wrap_behavior: c_int,
    pub update_initial_durations_done: c_int,
    pub pts_reorder_error: [i64; 17],
    pub pts_reorder_error_count: [u8; 17],
    pub last_dts_for_order_check: i64,
    pub dts_ordered: u8,
    pub dts_misordered: u8,
    pub inject_global_side_data: c_int,
    pub recommended_encoder_configuration: *mut c_char,
    pub display_aspect_ratio: AVRational,
}

#[link(name="avcodec")]
extern {
    pub fn avcodec_version() -> c_uint;
    pub fn avcodec_register_all();
    pub fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
    pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    pub fn avcodec_open2(avctx: *mut AVCodecContext,
                         codec: *const AVCodec,
                         options: *mut *mut AVDictionary)
                         -> c_int;
    pub fn avcodec_decode_video2(avctx: *mut AVCodecContext,
                                 picture: *mut AVFrame,
                                 got_picture_ptr: *mut c_int,
                                 avpkt: *const AVPacket)
                                 -> c_int;
    pub fn avcodec_decode_audio4(avctx: *mut AVCodecContext,
                                 frame: *mut AVFrame,
                                 got_frame_ptr: *mut c_int,
                                 avpkt: *const AVPacket)
                                 -> c_int;
    pub fn av_codec_set_pkt_timebase(avctx: *mut AVCodecContext, val: AVRational);
    pub fn avcodec_default_get_buffer(s: *mut AVCodecContext, frame: *mut AVFrame) -> c_int;
    pub fn av_init_packet(packet: *mut AVPacket);
    pub fn av_frame_alloc() -> *mut AVFrame;
    pub fn av_frame_free(frame: *mut *mut AVFrame);
    pub fn avcodec_get_frame_defaults(frame: *mut AVFrame);

    pub fn av_audio_resample_init(output_channels: c_int,
                                  input_channels: c_int,
                                  output_rate: c_int,
                                  input_rate: c_int,
                                  sample_fmt_out: AVSampleFormat,
                                  sample_fmt_in: AVSampleFormat,
                                  filter_length: c_int,
                                  log2_phase_count: c_int,
                                  linear: c_int,
                                  cutoff: c_double)
                                  -> *mut ReSampleContext;
    pub fn audio_resample(s: *mut ReSampleContext,
                          output: *mut c_short,
                          input: *mut c_short,
                          nb_samples: c_int)
                          -> c_int;
    pub fn audio_resample_close(s: *mut ReSampleContext);
}

#[link(name="avutil")]
extern {
    pub fn av_dict_free(m: *mut *mut AVDictionary);
    pub fn av_dict_set(pm: *mut *mut AVDictionary,
                       key: *const c_char,
                       value: *const c_char,
                       flags: c_int)
                       -> c_int;
    pub fn av_frame_get_plane_buffer(frame: *mut AVFrame, plane: c_int) -> *mut AVBufferRef;
    pub fn av_opt_get_double(obj: *mut c_void,
                             name: *const c_char,
                             search_flags: c_int,
                             out_val: *mut c_double)
                             -> c_int;
    pub fn av_opt_get_q(obj: *mut c_void,
                        name: *const c_char,
                        search_flags: c_int,
                        out_val: *mut AVRational)
                        -> c_int;
    pub fn av_samples_get_buffer_size(linesize: *mut c_int,
                                      nb_channels: c_int,
                                      nb_samples: c_int,
                                      sample_fmt: AVSampleFormat,
                                      align: c_int)
                                      -> c_int;

    pub fn av_malloc(size: size_t) -> *mut c_void;
    pub fn av_free(ptr: *mut c_void);
}

pub type AVIOPacketFn = extern "C" fn(opaque: *mut c_void,
                                      buf: *mut u8,
                                      buf_size: c_int)
                                      -> c_int;
pub type AVIOSeekFn = extern "C" fn(opaque: *mut c_void,
                                    offset: i64,
                                    whence: c_int)
                                    -> i64;

#[link(name="avformat")]
extern {
    pub fn av_register_all();
    pub fn avformat_version() -> c_uint;

    pub fn avformat_open_input(ps: *mut *mut AVFormatContext,
                               filename: *const c_char,
                               fmt: *mut AVInputFormat,
                               options: *mut *mut AVDictionary)
                               -> c_int;
    pub fn avformat_find_stream_info(ic: *mut AVFormatContext,
                                     options: *mut *mut AVDictionary)
                                     -> c_int;
    pub fn avformat_close_input(s: *mut *mut AVFormatContext);

    pub fn avformat_alloc_context() -> *mut AVFormatContext;
    pub fn avformat_free_context(s: *mut AVFormatContext);

    pub fn avio_alloc_context(buffer: *mut u8,
                              buffer_size: c_int,
                              write_flag: c_int,
                              opaque: *mut c_void,
                              read_packet: Option<AVIOPacketFn>,
                              write_packet: Option<AVIOPacketFn>,
                              seek: Option<AVIOSeekFn>)
                              -> *mut AVIOContext;

    pub fn av_read_frame(s: *mut AVFormatContext, pkg: *mut AVPacket) -> c_int;
}
