use serde::{Serialize, Deserialize};
use serde::{Serializer, Deserializer};
use std::fs::{File};
use std::io::Read;
use std::time::Duration;
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use toml::to_string as to_toml;
use serde_yaml::to_string as to_yaml;

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     name: String,
//     email: String,
//     birthday: String
// }

// fn main() {
//     let user = User{
//         name: "Denys Shvachka".to_string(),
//         email: "d.shvachka@ukma.edu.ua".to_string(),
//         birthday: "2014-01-01".to_string()
//     };
//     let json = serde_json::to_string(&user).unwrap();
//     println!("{}", json);
//
//     let deserialized: User = serde_json::from_str(&json).unwrap();
//     println!("{:?}", deserialized);
// }

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with="humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with="humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with="humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

fn main() {

    let event = Event {
        name: "Event meow".to_string(),
        date: "2005-06-2005".to_string(),
    };

    let json = serde_json::to_string_pretty(&event).unwrap();
    println!("{}", json);

    let deserialized: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", deserialized);

    // let mut file = File::open("request.json").unwrap();
    // let mut json_str = String::new();
    // file.read_to_string(&mut json_str).unwrap();
    // let request: Request = serde_json::from_str(&json_str).unwrap();
    // let yaml_str = to_yaml(&request).unwrap();
    // let toml_str = to_toml(&request).unwrap();
    // println!("Yaml:\n {}", yaml_str);
    // println!("\nToml:\n {}", toml_str);
}


#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with="serialize_date",
        deserialize_with="deserialize_date"
    )]
    date: String,
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    let res = data.replace("Date: ", "");
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        let request: Request = serde_json::from_str(&json_str).unwrap();

        assert_eq!(request.stream.user_id, Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap());
        assert_eq!(request.debug.duration, Duration::from_millis(234));
        assert_eq!(request.request_type, RequestType::Success);
        assert_eq!(request.stream.shard_url, Url::parse("https://n3.example.com/sapi").unwrap());
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[1].id, 2);
    }
}