// Idiom #102 Load from HTTP GET request into a file
// Make an HTTP request with method GET to the URL u, then store the body of the response in the file result.txt. Try to save the data as it arrives if possible, without having all its content in memory at once.
use std::fs::File;
use std::io::Write;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个 reqwest::Client
    let client = reqwest::Client::new();

    // 发起 GET 请求并等待响应
    let response = client.get("http://httpbin.org/get").send().await?;

    // 检查响应状态码
    if response.status().is_success() {
        // 读取响应体的字节数据
        let body_bytes = response.bytes().await?;

        // 打开文件以写入数据
        let mut file = File::create("example.html")?;

        // 将响应数据写入文件
        file.write_all(&body_bytes)?;

        println!("网页数据已成功写入文件。");
    } else {
        println!("HTTP请求失败: {:?}", response.status());
    }


    std::fs::remove_file("example.html")?;

    Ok(())
}
