use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub struct BgmPlayer {
    _stream: OutputStream,
    sink: Sink,
    is_playing: bool,
    current_bgm: String,
}

impl BgmPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        BgmPlayer {
            _stream: stream,
            sink,
            is_playing: false,
            current_bgm: String::new(),
        }
    }

    pub fn play_bgm(&mut self, bgm_path: String) {
        // 初态: 没有BGM在播放
        if !self.is_playing && self.current_bgm == String::new() {
            self.play(bgm_path.clone());
        }

        // 模态转换: 给定的BGM不是当前正在播放的BGM 则播放给定的BGM
        if self.is_playing && self.current_bgm != bgm_path.clone() {
            // step1. 停止当前播放的BGM
            self.stop();

            // step2. 播放新的BGM
            self.play(bgm_path.clone());
        }

        if self.sink.empty() {
            self.is_playing = false;
            self.current_bgm = String::new();
        }
    }

    fn play(&mut self, bgm_path: String) {
        let file = File::open(bgm_path.as_str()).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
        self.is_playing = true;
        self.current_bgm = bgm_path.clone();
    }

    fn stop(&mut self)  {
        self.sink.stop();
        self.is_playing = false;
        self.current_bgm = String::new();
    }
}

impl Default for BgmPlayer {
    fn default() -> Self {
        Self::new()
    }
}