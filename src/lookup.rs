extern crate serde;
extern crate serde_json;

extern crate trust_dns;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::Message;
use trust_dns::rr::{DNSClass, Name, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use std::str::FromStr;

#[derive(Serialize, Debug)]
pub struct WRequest {
    name: String,
    dns_server: String,
    atype: String,
}

#[derive(Serialize, Debug)]
pub struct QResult {
    name: String,
    address: String,
    atype: String,
}

#[derive(Serialize, Debug)]
pub struct QResponse {
    results: Vec<QResult>,
}

#[derive(Serialize, Debug)]
pub struct LookupConfig {
    pub dns_server: String,
    pub dns_port: String,
    pub atype: String,
}


pub fn check_ip4(hostname: &String, config: &LookupConfig) -> Message{
    let name = Name::from_str(*&hostname).unwrap();
    let address = format!("{}:{}", config.dns_server, config.dns_port).parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    return response;

}

pub fn check_ip6(hostname: &String, config: &LookupConfig ) -> Message{
    let name = Name::from_str(&*hostname).unwrap();
    let address = format!("{}:{}", config.dns_server, config.dns_port).parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let response: Message = client.query(&name, DNSClass::IN, RecordType::AAAA).unwrap();
    return response;
}

pub fn extract_results(response: &Message) -> QResponse {
    // let mut results = Vec<QResult>::new();
    let mut results : Vec<QResult> = vec![];

    let answers: &[Record] = response.answers();
    if answers.len() > 0 {
        for ans in answers {
            let rdata = ans.rdata();
            if let RecordType::A = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                println!("{} {}", ans.name(), (ip).to_string());
                let result = QResult {
                    atype: "ip4".to_string(),
                    name: ans.name().to_string().clone(),
                    address: (ip).to_string().clone(),
                };
                results.push(result);

            } else if let RecordType::AAAA = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                println!("{} {}", ans.name(), (ip).to_string());                      
            } 
        }
    }
    let r = QResponse {results: results};
    return r;
}