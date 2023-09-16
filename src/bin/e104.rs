// Idiom #104 Save object into XML file
// Write the contents of the object x into the file data.xml.

use serde::Serialize;
use std::fs::File;
use std::io::Write;

// 创建一个结构体来表示XML数据的解析结果
#[derive(Debug, Serialize)]
struct Person {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Age")]
    age: u32,
}


fn main() {
    // 创建一个Person结构体的实例
    let person = Person {
        name: "John".to_string(),
        age: 42,
    };

    // 将Person结构体实例转换为XML字符串
    let xml_data = serde_xml_rs::to_string(&person).unwrap();

    // 打印XML字符串
    println!("{}", xml_data);

    // 将XML字符串写入文件
    let mut file = File::create("example.xml").unwrap();
    file.write_all(xml_data.as_bytes()).unwrap(); 
}