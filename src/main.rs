use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DnsRecord {
    pub content: String,
    pub name: String,
    pub comment: String,
    pub proxied: bool,
    pub r#type: String,
    pub id: String,
    pub ttl: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    api_key: String,
    zone_id: String,
    records: Vec<DnsRecord>,
}

fn main() {
    let content = std::fs::read_to_string("./ddns.json").unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();

    println!("{config:#?}");

    for record in config.records {
        let data = serde_json::to_string(&record).unwrap();
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            config.zone_id, record.id
        );
        let res = ureq::put(&url)
            .set("Content-Type", "application/json")
            .set("Authorization", &format!("Bearer {}", config.api_key))
            .send_string(&data)
            .unwrap() // FIXME
            .into_string()
            .unwrap(); // FIXME

        println!("{res}")
    }
}
