// Idiom #103 Load XML file into struct
// Read from the file data.xml and write its contents into the object x.
// Assume the XML data is suitable for the type of x.

use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs::File;
use std::io::Read;

// 创建一个结构体来表示XML数据的解析结果
#[derive(Debug, Deserialize)]
struct Person {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Age")]
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开XML文件以进行读取
    let mut file = File::open("example.xml")?;
    
    // 创建一个字符串来存储XML数据
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data)?;

    // 解析XML数据
    let person: Person = from_str(&xml_data)?;

    // 打印解析结果
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);

    Ok(())
}
