use std::old_io::{ Reader, IoResult, IoError, EndOfFile };
use std::mem::size_of;
use std::marker::PhantomData;
use libc::c_int;

use util::{ SampleFormat, SampleFormatType };
use avformat::{ AvFormatContext, AvioContext, AvioContextReader };
use avcodec::{ AvCodecContext, AvPacket };
use avutil::{ AvFrame, AvDictionary };
use ffi::{ AVMEDIA_TYPE_AUDIO };

pub struct AudioDecoder<'a, R, T = i16> {
    context: AvFormatContext<AvioContext<AvioContextReader<R>>>,
    decoder: AvCodecContext,
    packet: AvPacket<'a>,
    frame: AvFrame,
    frame_size: usize,
    frame_offset: usize,
    pub position: u64,
    _marker: PhantomData<T>,
}

impl<'a, R: Reader + 'a, T: SampleFormatType> AudioDecoder<'a, R, T> {
    pub fn new(reader: R) -> Result<Self, c_int> {
        let avio = AvioContext::new(0x1000, false, AvioContextReader::new(reader));
        let (context, _) = AvFormatContext::open_input(avio, AvDictionary::new());
        let context = try!(context);

        let (res, _) = context.find_stream_info(AvDictionary::new());
        try!(res);

        let stream_index = try!(context.find_stream(AVMEDIA_TYPE_AUDIO).ok_or(-1));
        let (decoder, _) = context.open_stream(stream_index, AvDictionary::new());
        let decoder = try!(decoder);
        unsafe {
            *ffmpeg_ffi_avcodeccontext_field!(decoder.context, mut sample_fmt) = SampleFormat::from_type::<T>().sample_fmt();
        }

        Ok(AudioDecoder {
            context: context,
            decoder: decoder,
            packet: AvPacket::empty(),
            frame: AvFrame::new(),
            frame_size: 0,
            frame_offset: 0,
            position: 0,
            _marker: PhantomData,
        })
    }

    fn read_frame(&mut self) -> Result<(), ()> {
        if self.packet.has_data() {
            try!(self.decoder.decode_audio(&self.frame, &mut self.packet));

            self.frame_offset = 0;
            self.frame_size = (self.frame.sample_count() * self.decoder.channels()) as usize * size_of::<T>();
        } else {
            try!(self.context.read_packet(&mut self.packet).or_else(|_| Err(())));
        }

        Ok(())
    }

    fn fill_buffer(&mut self) -> Result<(), ()> {
        while self.frame_offset >= self.frame_size {
            try!(self.read_frame());
        }

        Ok(())
    }

    fn update_position(&mut self, sz: usize) {
        self.frame_offset += sz;
        self.position += sz as u64 / size_of::<T>() as u64;
    }

    pub fn channels(&self) -> usize {
        self.decoder.channels() as usize
    }

    pub fn sample_rate(&self) -> usize {
        self.decoder.sample_rate() as usize
    }

    pub fn buffer_size(&self) -> usize {
        self.frame_size - self.frame_offset
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer_size() / self.channels()
    }
}

impl<'a, R: Reader + 'a, T: SampleFormatType> Reader for AudioDecoder<'a, R, T> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let sz = {
            let data = try!(self.fill_buf());
            buf.clone_from_slice(data)
        };
        self.consume(sz);

        Ok(sz)
    }
}

impl<'a, R: Reader + 'a, T: SampleFormatType> Buffer for AudioDecoder<'a, R, T> {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        try!(self.fill_buffer().or_else(|_| Err(IoError { kind: EndOfFile, desc: "end of file", detail: None })));

        Ok(&self.frame.audio_data(0, self.decoder.channels())[self.frame_offset..])
    }

    fn consume(&mut self, amt: usize) {
        self.update_position(amt);
    }
}

impl<'a, R: Reader + 'a, T: SampleFormatType + Copy> Iterator for AudioDecoder<'a, R, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe {
            let sz = size_of::<T>();
            let v = if let Ok(data) = self.fill_buf() {
                if data.len() >= sz {
                    *(data.as_ptr() as *const T)
                } else {
                    return None
                }
            } else {
                return None
            };
            self.consume(sz);
            Some(v)
        }
    }
}
