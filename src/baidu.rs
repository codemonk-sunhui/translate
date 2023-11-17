use reqwest;
use std::error::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};


pub async fn baidu_translate(q: &String) -> Result<Payload, Box<dyn Error>> {
    let sign = get_sign(q);
    let to = get_to(&q);
    let url = format!("https://fanyi-api.baidu.com/api/trans/vip/translate?q={q}&from=auto&to={to}&appid=20231116001882443&salt=123&sign={sign}");
    println!("{}", url);
    let client = reqwest::Client::new();
    let doge = client
        .get(url)
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json::<Payload>()
        .await?;

    println!("{}", doge.from);
    Ok(doge)
}

fn md5(input: String) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn get_sign(q: &String) -> String {
    let text = format!("20231116001882443{q}123eRDq8OYvBUCJzu9YRHwu");
    md5(text)
}

fn get_to(q: &String) -> String {
    let c = q.chars().next().expect("none");
    return if is_alphabetic(&c) {
        String::from("zh")
    } else {
        String::from("en")
    };
}

pub fn is_alphabetic(c: &char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        _ => false
    }
}


#[derive(Serialize, Deserialize)]
pub struct TransResult {
    pub src: String,
    pub dst: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<TransResult>,
}


