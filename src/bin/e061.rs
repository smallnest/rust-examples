// Idiom #61 Get current date
// Assign to the variable d the current date/time value, in the most standard type.


use std::time::SystemTime;
use chrono::{Utc, Local};


fn main() {
    let d = SystemTime::now();
    println!("{:?}", d);

    // 获取当前时间戳（UTC）
    let utc_now = Utc::now();
    println!("UTC now: {}", utc_now);

    // 获取当前时间戳（本地时间）
    let local_now = Local::now();
    println!("Local now: {}", local_now);

    // 格式化时间戳
    let formatted_utc = utc_now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted UTC: {}", formatted_utc);

    let formatted_local = local_now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted local: {}", formatted_local);
}

