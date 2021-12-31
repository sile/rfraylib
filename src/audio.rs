use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct AudioDevice(pub(crate) ()); // TODO

impl AudioDevice {
    pub(crate) fn new() -> Self {
        // TODO: check duplicate
        unsafe { raylib4_sys::InitAudioDevice() };
        Self(())
    }

    /// Set master volume (listener).
    pub fn set_master_volume(&mut self, volume: f32) {
        unsafe { raylib4_sys::SetMasterVolume(volume) };
    }

    /// Play a sound.
    pub fn play_sound(&mut self, sound: &Sound) {
        unsafe { raylib4_sys::PlaySound(sound.0) };
    }

    /// Stop playing a sound.
    pub fn stop_sound(&mut self, sound: &Sound) {
        unsafe { raylib4_sys::StopSound(sound.0) };
    }

    /// Pause a sound.
    pub fn pause_sound(&mut self, sound: &Sound) {
        unsafe { raylib4_sys::PauseSound(sound.0) };
    }

    /// Resume a paused sound.
    pub fn resume_sound(&mut self, sound: &Sound) {
        unsafe { raylib4_sys::ResumeSound(sound.0) };
    }

    /// Play a sound (using multichannel buffer pool).
    pub fn play_sound_multi(&mut self, sound: &Sound) {
        unsafe { raylib4_sys::PlaySoundMulti(sound.0) };
    }

    /// Stop any sound playing (using multichannel buffer pool).
    pub fn stop_sound_multi(&mut self) {
        unsafe { raylib4_sys::StopSoundMulti() };
    }

    /// Get number of sounds playing in the multichannel.
    pub fn get_sounds_playing(&self) -> usize {
        unsafe { raylib4_sys::GetSoundsPlaying() as usize }
    }

    /// Check if a sound is currently playing.
    pub fn is_sound_playing(&self, sound: &Sound) -> bool {
        unsafe { raylib4_sys::IsSoundPlaying(sound.0) }
    }

    /// Set volume for a sound (1.0 is max level).
    pub fn set_sound_volume(&mut self, sound: &Sound, volume: f32) {
        unsafe { raylib4_sys::SetSoundVolume(sound.0, volume) };
    }

    /// Set pitch for a sound (1.0 is base level).
    pub fn set_sound_pitch(&mut self, sound: &Sound, pitch: f32) {
        unsafe { raylib4_sys::SetSoundPitch(sound.0, pitch) };
    }

    /// Start music playing.
    pub fn play_music_stream(&mut self, music: &Music) {
        unsafe { raylib4_sys::PlayMusicStream(music.0) };
    }

    /// Check if music is playing.
    pub fn is_music_stream_playing(&mut self, music: &Music) -> bool {
        unsafe { raylib4_sys::IsMusicStreamPlaying(music.0) }
    }

    /// Updates buffers for music streaming.
    pub fn update_music_stream(&mut self, music: &Music) {
        unsafe { raylib4_sys::UpdateMusicStream(music.0) };
    }

    /// Stop music playing.
    pub fn stop_music_stream(&mut self, music: &Music) {
        unsafe { raylib4_sys::StopMusicStream(music.0) };
    }

    /// Pause music playing.
    pub fn pause_music_stream(&mut self, music: &Music) {
        unsafe { raylib4_sys::PauseMusicStream(music.0) };
    }

    /// Resume playing paused music.
    pub fn resume_music_stream(&mut self, music: &Music) {
        unsafe { raylib4_sys::ResumeMusicStream(music.0) };
    }

    /// Seek music to a position (in seconds).
    pub fn seek_music_stream(&mut self, music: &Music, position: Duration) {
        unsafe { raylib4_sys::SeekMusicStream(music.0, position.as_secs_f32()) };
    }

    /// Set volume for music (1.0 is max level).
    pub fn set_music_volume(&mut self, music: &Music, volume: f32) {
        unsafe { raylib4_sys::SetMusicVolume(music.0, volume) };
    }

    /// Set pitch for a music (1.0 is base level).
    pub fn set_music_pitch(&mut self, music: &Music, pitch: f32) {
        unsafe { raylib4_sys::SetMusicPitch(music.0, pitch) };
    }

    /// Get current music time played (in seconds).
    pub fn get_music_time_played(&self, music: &Music) -> Duration {
        let s = unsafe { raylib4_sys::GetMusicTimePlayed(music.0) };
        Duration::from_secs_f32(s)
    }

    /// Play audio stream.
    pub fn play_audio_stream(&mut self, stream: &AudioStream) {
        unsafe { raylib4_sys::PlayAudioStream(stream.0) };
    }

    /// Pause audio stream.
    pub fn pause_audio_stream(&mut self, stream: &AudioStream) {
        unsafe { raylib4_sys::PauseAudioStream(stream.0) };
    }

    /// Resume audio stream.
    pub fn resume_audio_stream(&mut self, stream: &AudioStream) {
        unsafe { raylib4_sys::ResumeAudioStream(stream.0) };
    }

    /// Stop audio stream.
    pub fn stop_audio_stream(&mut self, stream: &AudioStream) {
        unsafe { raylib4_sys::StopAudioStream(stream.0) };
    }

    /// Check if audio stream is playing.
    pub fn is_audio_stream_playing(&mut self, stream: &AudioStream) -> bool {
        unsafe { raylib4_sys::IsAudioStreamPlaying(stream.0) }
    }

    /// Set volume for audio stream (1.0 is max level).
    pub fn set_audio_stream_volume(&mut self, stream: &AudioStream, volume: f32) {
        unsafe { raylib4_sys::SetAudioStreamVolume(stream.0, volume) };
    }

    /// Set pitch for audio stream (1.0 is base level).
    pub fn set_audio_stream_pitch(&mut self, stream: &AudioStream, pitch: f32) {
        unsafe { raylib4_sys::SetAudioStreamPitch(stream.0, pitch) };
    }

    /// Default size for new audio streams.
    pub fn set_audio_stream_buffer_size_default(&mut self, size: usize) {
        unsafe { raylib4_sys::SetAudioStreamBufferSizeDefault(size as c_int) };
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        unsafe { raylib4_sys::CloseAudioDevice() };
    }
}

#[derive(Debug)]
pub struct Wave(raylib4_sys::Wave);

impl Wave {
    /// Load wave data from file.
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let wave = unsafe { raylib4_sys::LoadWave(path.as_ptr()) };
        if wave.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(wave))
        }
    }

    /// Load wave from memory buffer.
    pub fn load_from_memory(file_type: &str, file_data: &[u8]) -> Option<Self> {
        let file_type = CString::new(file_type).ok()?;
        let wave = unsafe {
            raylib4_sys::LoadWaveFromMemory(
                file_type.as_ptr(),
                file_data.as_ptr(),
                file_data.len() as c_int,
            )
        };
        if wave.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(wave))
        }
    }

    /// Export wave data to file, returns true on success.
    pub fn export<P: AsRef<Path>>(&self, path: P) -> bool {
        path_to_cstring(path)
            .map(|path| unsafe { raylib4_sys::ExportWave(self.0, path.as_ptr()) })
            .unwrap_or(false)
    }

    /// Convert wave data to desired format.
    pub fn format(&mut self, sample_rate: usize, sample_size: usize, channels: usize) {
        unsafe {
            raylib4_sys::WaveFormat(
                &mut self.0,
                sample_rate as c_int,
                sample_size as c_int,
                channels as c_int,
            )
        };
    }

    /// Crop a wave to defined samples range.
    pub fn crop(&mut self, init_sample: usize, final_sample: usize) {
        unsafe { raylib4_sys::WaveCrop(&mut self.0, init_sample as c_int, final_sample as c_int) };
    }

    /// Load samples data from wave as a floats array.
    ///
    /// NOTE: Returned sample values are normalized to range [-1..1]
    pub fn to_samples(&self) -> Vec<f32> {
        let samples_ptr = unsafe { raylib4_sys::LoadWaveSamples(self.0) };
        let n = (self.0.frameCount * self.0.channels) as usize;
        let samples = (0..n)
            .map(|i| unsafe { &*std::ptr::slice_from_raw_parts(samples_ptr, n) }[i])
            .collect();
        unsafe { raylib4_sys::UnloadWaveSamples(samples_ptr) };
        samples
    }
}

impl Clone for Wave {
    fn clone(&self) -> Self {
        Self(unsafe { raylib4_sys::WaveCopy(self.0) })
    }
}

impl Drop for Wave {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadWave(self.0) };
    }
}

#[derive(Debug)]
pub struct Sound(raylib4_sys::Sound);

impl Sound {
    /// Load sound data from file.
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let sound = unsafe { raylib4_sys::LoadSound(path.as_ptr()) };
        if sound.stream.buffer == std::ptr::null_mut() {
            None
        } else {
            Some(Self(sound))
        }
    }

    /// Load sound from wave data.
    pub fn load_from_save(wave: Wave) -> Option<Self> {
        let sound = unsafe { raylib4_sys::LoadSoundFromWave(wave.0) };
        if sound.stream.buffer == std::ptr::null_mut() {
            None
        } else {
            Some(Self(sound))
        }
    }

    /// Update sound buffer with new data.
    pub fn update(&mut self, data: &[u8], sample_count: usize) {
        unsafe {
            raylib4_sys::UpdateSound(
                self.0,
                data.as_ptr() as *const c_void,
                sample_count as c_int,
            )
        };
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            raylib4_sys::StopSound(self.0);
            raylib4_sys::UnloadSound(self.0);
        }
    }
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Option<CString> {
    path.as_ref().to_str().and_then(|p| CString::new(p).ok())
}

#[derive(Debug)]
pub struct Music(raylib4_sys::Music);

impl Music {
    /// Load music stream from file.
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let music = unsafe { raylib4_sys::LoadMusicStream(path.as_ptr()) };
        if music.ctxData == std::ptr::null_mut() {
            None
        } else {
            Some(Self(music))
        }
    }

    /// Load music stream from data.
    pub fn load_from_memory(file_type: &str, data: &[u8]) -> Option<Self> {
        let file_type = CString::new(file_type).ok()?;
        let music = unsafe {
            raylib4_sys::LoadMusicStreamFromMemory(
                file_type.as_ptr(),
                data.as_ptr() as *mut u8,
                data.len() as c_int,
            )
        };
        if music.ctxData == std::ptr::null_mut() {
            None
        } else {
            Some(Self(music))
        }
    }

    /// Get music time length (in seconds).
    pub fn get_time_length(&self) -> Duration {
        let s = unsafe { raylib4_sys::GetMusicTimeLength(self.0) };
        Duration::from_secs_f32(s)
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            raylib4_sys::StopMusicStream(self.0);
            raylib4_sys::UnloadMusicStream(self.0);
        }
    }
}

#[derive(Debug)]
pub struct AudioStream(raylib4_sys::AudioStream);

impl AudioStream {
    /// Load audio stream (to stream raw audio pcm data).
    pub fn load(sample_rate: usize, sample_size: usize, channels: usize) -> Self {
        Self(unsafe {
            raylib4_sys::LoadAudioStream(sample_rate as u32, sample_size as u32, channels as u32)
        })
    }

    /// Update audio stream buffers with data.
    ///
    /// - NOTE 1: Only updates one buffer of the stream source: unqueue -> update -> queue
    /// - NOTE 2: To unqueue a buffer it needs to be processed: IsAudioStreamProcessed()
    pub fn update(&mut self, data: &[u8], samples_count: usize) {
        unsafe {
            raylib4_sys::UpdateAudioStream(
                self.0,
                data.as_ptr() as *const c_void,
                samples_count as c_int,
            )
        };
    }

    /// Check if any audio stream buffers requires refill.
    pub fn is_processed(&self) -> bool {
        unsafe { raylib4_sys::IsAudioStreamProcessed(self.0) }
    }
}

impl Drop for AudioStream {
    fn drop(&mut self) {
        unsafe {
            raylib4_sys::StopAudioStream(self.0);
            raylib4_sys::UnloadAudioStream(self.0);
        }
    }
}
