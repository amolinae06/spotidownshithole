use std::io::{Read, Seek};
use lame_sys::{MPEG_mode, lame_close, lame_encode_buffer, lame_encode_flush, lame_global_flags, 
    lame_init, lame_init_params, lame_set_VBR, lame_set_VBR_hard_min, lame_set_VBR_min_bitrate_kbps, 
    lame_set_VBR_q, lame_set_VBR_quality, lame_set_in_samplerate, lame_set_mode, 
    lame_set_num_channels, lame_set_preset, vbr_default};
use lewton::inside_ogg::OggStreamReader;

use crate::downloader::{AudioFormat, Quality};
use crate::error::SpotifyError;

/// Converts audio to MP3
pub enum AudioConverter {
    OGG {
        decoder: OggStreamReader<ReadWrap>,
        lame: *mut lame_global_flags,
        lame_end: bool
    }
}

unsafe impl Send for AudioConverter {}

impl AudioConverter {
    /// Wrap reader
    pub fn new(read: Box<(dyn Read + Send + 'static)>, format: AudioFormat, quality: Quality) -> Result<AudioConverter, SpotifyError> {
        // Create encoder
        let bitrate = match quality {
            Quality::Q320 => 320,
            Quality::Q256 => 256,
            Quality::Q160 => 160,
            Quality::Q96 =>  96,
        };
        let lame = unsafe {
            let gfp = lame_init();
            lame_set_VBR(gfp, vbr_default);
            // 2 channels
            lame_set_num_channels(gfp, 0);
            // V0
            lame_set_preset(gfp, 500);
            // 0 = best
            lame_set_VBR_q(gfp, 0);
            lame_set_VBR_quality(gfp, 0.0);
            lame_set_mode(gfp, MPEG_mode::STEREO);
            // Enfore Bitrate
            lame_set_VBR_min_bitrate_kbps(gfp, bitrate);
            lame_set_VBR_hard_min(gfp, 1);
            gfp
        };

        match format {
            AudioFormat::AAC => todo!(),
            AudioFormat::MP4 => todo!(),
            // Lewton decoder
            AudioFormat::OGG => {
                let decoder = OggStreamReader::new(ReadWrap::new(Box::new(read)))?;
                let sample_rate = decoder.ident_hdr.audio_sample_rate;
                // Init lame
                unsafe {
                    lame_set_in_samplerate(lame, sample_rate as i32);
                    lame_init_params(lame);
                }
                return Ok(AudioConverter::OGG { lame, decoder, lame_end: false });
            },
            AudioFormat::MP3 => panic!("no reencoding allowd!"),
            _ => return Err(SpotifyError::InvalidFormat)
        };
    }
}

impl Read for AudioConverter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            AudioConverter::OGG { decoder, lame, lame_end } => {
                match decoder.read_dec_packet() {
                    Ok(packet) => match packet {
                        Some(mut data) => {
                            // 0 sized packets aren't EOF
                            if data[0].len() == 0 {
                                return self.read(buf);
                            }
    
                            // Encode
                            let n_samples = data[0].len();
                            let res = unsafe {
                                lame_encode_buffer(
                                    *lame,
                                    data[0].as_mut_ptr(),
                                    data[1].as_mut_ptr(),
                                    n_samples as i32,
                                    buf.as_mut_ptr(),
                                    0
                                )
                            };

                            // Shit broke
                            if res < 0 {
                                unsafe { lame_close(*lame); }
                                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Lame error: {}", res)))
                            }
                            // Encoded size can be 0
                            if res == 0 {
                                return self.read(buf)
                            }
                            Ok(res as usize)
                        },
                        None => {
                            if *lame_end {
                                return Ok(0);
                            }
                            // Flush buffer
                            let r = unsafe {
                                let r = lame_encode_flush(*lame, buf.as_mut_ptr(), 0);
                                lame_close(*lame);
                                r
                            };
                            *lame_end = true;
                            Ok(r as usize)
                        }
                    }
                    Err(e) => {
                        // Close lame
                        if !*lame_end {
                            unsafe { lame_close(*lame); }
                            *lame_end = true;
                        }
                        warn!("Lawton error: {}, calling EOF", e);
                        Ok(0)
                    }
                }
            },
        }

    }
}

pub struct ReadWrap {
    source: Box<(dyn Read + Send + 'static)>
}

impl ReadWrap {
    pub fn new(read: Box<(dyn Read + Send + 'static)>) -> ReadWrap{
        ReadWrap {
            source: Box::new(read)
        }
    }
}

impl Read for ReadWrap {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.source.read(buf)
    }
}

/// Fake seek for Rodio
impl Seek for ReadWrap {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Ok(0)
    }
}