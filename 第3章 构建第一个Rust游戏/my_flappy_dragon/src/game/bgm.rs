use std::fs::File;
use std::io::{BufReader};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream};
use crate::game::BgmSignal;

pub struct BgmPlayer {
    // OutputStream必须被保存 否则无法控制音频设备输出
    _stream: OutputStream,
    sink: Arc<Mutex<rodio::Sink>>,
    pub is_playing: Arc<Mutex<bool>>,
    current_bgm: Arc<Mutex<String>>,
    sender: Arc<Mutex<Sender<BgmSignal>>>,
    receiver: Arc<Mutex<Receiver<BgmSignal>>>,
}

impl BgmPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let (sender, receiver) = mpsc::channel();
        BgmPlayer {
            _stream,
            sink: Arc::new(Mutex::new(sink)),
            is_playing: Arc::new(Mutex::new(false)),
            current_bgm: Arc::new(Mutex::new(String::new())),
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn play(&mut self, bgm_path: String) {
        let bgm_path_clone = bgm_path.clone();
        let sink = Arc::clone(&self.sink);
        let receiver = Arc::clone(&self.receiver);

        // 初态: 没有BGM正在播放
        if !*self.is_playing.lock().unwrap() && *self.current_bgm.lock().unwrap() == String::new() {
            *self.is_playing.lock().unwrap() = true;
            *self.current_bgm.lock().unwrap() = bgm_path.clone();
            thread::spawn(move || {
                Self::play_bgm(sink, bgm_path_clone, receiver);
            });
        }

        // 有新的BGM需要播放 则发送停止信号
        if *self.current_bgm.lock().unwrap() != bgm_path {
            self.sender.lock().unwrap().send(BgmSignal::Stop).unwrap();
            *self.current_bgm.lock().unwrap() = bgm_path.clone();

            // 重新创建channel和播放子线程
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let (sender, receiver) = mpsc::channel();
            self._stream = _stream;
            *self.sink.lock().unwrap() = sink;
            *self.sender.lock().unwrap() = sender;
            *self.receiver.lock().unwrap() = receiver;
            let sink = Arc::clone(&self.sink);
            let receiver = Arc::clone(&self.receiver);

            thread::spawn(move || {
                Self::play_bgm(sink, bgm_path, receiver);
            });
        }
    }

    fn play_bgm(sink: Arc<Mutex<rodio::Sink>>, bgm_path: String, receiver: Arc<Mutex<Receiver<BgmSignal>>>) {
        // 播放BGM
        let file = File::open(bgm_path.as_str()).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        {
            let sink_guard = sink.lock().unwrap();
            sink_guard.append(source);
            println!("BgmPlayer::play_bgm() 开始播放BGM: {}", bgm_path);
            sink_guard.play();
        }

        loop {
            // 检测是否有终止信号
            if let Ok(BgmSignal::Stop) = receiver.lock().unwrap().try_recv() {
                println!("BgmPlayer::play_bgm() 收到停止信号，准备退出...");
                let sink_guard = sink.lock().unwrap();
                sink_guard.stop();
                sink_guard.clear();
                break;
            }

            // 检查当前Sink是否正在播放 如果在播放 则100ms后再次检测终止信号的到来
            if !sink.lock().unwrap().empty() {
                thread::sleep(Duration::from_millis(100));
            } else {
                // 如果当前Sink没有正在播放 且 没有收到终止信号 则再次播放BGM
                let file = File::open(bgm_path.as_str()).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.lock().unwrap().append(source);
            }
        }
    }
}