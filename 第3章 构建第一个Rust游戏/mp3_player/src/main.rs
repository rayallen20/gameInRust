use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // 将音频添加到Sink并循环播放
    loop {
        // 打开mp3文件
        let file = File::open("./music/under_water.mp3").unwrap();
        // 创建解码器
        let source = Decoder::new(BufReader::new(file)).unwrap();
        // 将音频添加到Sink
        sink.append(source);

        // 等待音频播放完成
        while !sink.empty() {
            thread::sleep(Duration::from_millis(100));
        }
    }

}
