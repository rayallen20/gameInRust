// use sysinfo::System;
// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() < 2 {
//         println!("Usage: {} <process_name>", args[0]);
//         return;
//     }
//     let process_name = &args[1];
//
//     // 初始化系统对象
//     let mut system = System::new_all();
//
//     // 更新系统信息
//     system.refresh_all();
//
//     // 查找给定名称的进程
//     let mut found = false;
//     for (pid, process) in system.processes() {
//         if process.name().to_string_lossy() == process_name.as_str() {
//            let thread_count = match process.tasks() {
//                Some(tasks) => tasks.len(),
//                None => 0,
//            };
//
//            println!("Process ID: {}, Name: {}, Thread count: {}", pid, process.name().to_string_lossy(), thread_count);
//            found = true;
//         }
//     }
//
//     if !found {
//         println!("Process {} not found", process_name);
//     }
// }

use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // let process_name = "my_flappy_name"; // 替换为你的进程名称
    //
    // let output = Command::new("sh")
    //     .arg("-c")
    //     .arg(format!("ps -e -T | grep {}", process_name))
    //     .output()
    //     .expect("Failed to execute command");
    //
    // let stdout = String::from_utf8_lossy(&output.stdout);
    // let thread_count = stdout.lines().count();
    //
    // println!("Process Name: {}, Thread count: {}", process_name, thread_count);

    for i in 0..5 {
        println!("fuck");
        thread::sleep(Duration::from_secs(5));
    }

}
