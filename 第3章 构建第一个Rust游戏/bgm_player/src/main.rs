use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

pub struct BgmPlayer {
    _stream: OutputStream, // 确保 OutputStream 在 BgmPlayer 的生命周期内有效
    sink: Arc<Mutex<Sink>>,
    pub is_playing: Arc<Mutex<bool>>,
}

impl BgmPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        BgmPlayer {
            _stream: stream, // 保存 OutputStream
            sink: Arc::new(Mutex::new(sink)),
            is_playing: Arc::new(Mutex::new(false)),
        }
    }

    pub fn play(&mut self, bgm_path: String) {
        let is_playing = Arc::clone(&self.is_playing);
        let sink = Arc::clone(&self.sink);

        if !*is_playing.lock().unwrap() {
            *is_playing.lock().unwrap() = true;
            thread::spawn(move || {
                loop {
                    let file = File::open(&bgm_path).unwrap();
                    let source = Decoder::new(BufReader::new(file)).unwrap();
                    {
                        let mut sink_guard = sink.lock().unwrap();
                        sink_guard.append(source);
                    }
                    {
                        let sink_guard = sink.lock().unwrap();
                        sink_guard.sleep_until_end();
                    }
                }
            });
        }
    }
}

fn main() {
    let mut bgm_player = BgmPlayer::new();
    bgm_player.play("./music/mario_3.mp3".to_string());

    for i in 0..=5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Main thread finished.");
}
