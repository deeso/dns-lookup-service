extern crate serde;
extern crate serde_json;

extern crate trust_dns;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::Message;
use trust_dns::rr::{DNSClass, Name, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use std::str::FromStr;

use config;

#[derive(Debug, Clone)]
pub struct DnsLookupService {
    pub dns_server: String,
    pub dns_port: String,
    pub use_tls: bool,
    pub ip4: bool,
    pub ip6: bool,
}

#[derive(Debug, Clone)]
pub struct DnsLookupServices {
    pub servicers: Vec<DnsLookupService>,
}

#[derive(Serialize, Debug)]
pub struct DnsLookupRequest {
    name: String,
    dns_server: String,
    atype: String,
}

#[derive(Serialize, Debug)]
pub struct DnsLookupResult {
    name: String,
    address: String,
    atype: String,
}

#[derive(Serialize, Debug)]
pub struct DnsLookupResults {
    results: Vec<DnsLookupResult>,
}

fn extract_results(response: &Message) -> DnsLookupResults {
    // let mut results = Vec<DnsLookupResult>::new();
    let mut results : Vec<DnsLookupResult> = vec![];

    let answers: &[Record] = response.answers();
    if answers.len() > 0 {
        for ans in answers {
            let rdata = ans.rdata();
            if let RecordType::A = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                println!("{} {}", ans.name(), (ip).to_string());
                let result = DnsLookupResult {
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
    let r = DnsLookupResults {results: results};
    return r;
}

impl DnsLookupService {

    pub fn from_service_config(dsc: &config::DnsServerConfig) -> Option<DnsLookupService> {
        return Some(DnsLookupService {
            dns_server: dsc.nameserver.clone(),
            dns_port: "53".to_string(),
            use_tls: false,
            ip4: dsc.ip4.clone(),
            ip6: dsc.ip6.clone(),
        })
    }

    pub fn check(&self, hostname: &String) -> DnsLookupResults{
        let mut results : Vec<DnsLookupResult> = vec![];
        if self.ip4 {
            let response = self.check_ip4(&hostname);
            let dns_results = extract_results(&response);
            for r in dns_results.results {
                results.push(r)
            }
        } 
        if self.ip6 {
            let response = self.check_ip6(&hostname);
            let dns_results = extract_results(&response);
            for r in dns_results.results {
                results.push(r)
            }
        }
        let dlr = DnsLookupResults {results: results};
        return dlr;    
    }

    fn check_ip4(&self, hostname: &String) -> Message{
        let name = Name::from_str(*&hostname).unwrap();
        let address = format!("{}:{}", self.dns_server, self.dns_port).parse().unwrap();
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
        return response;

    }

    fn check_ip6(&self, hostname: &String) -> Message{
        let name = Name::from_str(&*hostname).unwrap();
        let address = format!("{}:{}", self.dns_server, self.dns_port).parse().unwrap();
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let response: Message = client.query(&name, DNSClass::IN, RecordType::AAAA).unwrap();
        return response;
    }

}

impl DnsLookupServices {
    pub fn from_service_configs(dsc: &config::DnsServerConfigs) -> Option<DnsLookupServices> {
        let mut results : Vec<DnsLookupService> = vec![];
        for c in dsc.servers {
            let dls = DnsLookupService {
                dns_server: c.nameserver.clone(),
                dns_port: "53".to_string(),
                use_tls: false,
                ip4: c.ip4.clone(),
                ip6: c.ip6.clone(),
            };
            results.push(dls);
        }
        return Some(DnsLookupServices{servicers:results})
    }

    pub fn check(&self, hostname: &String) -> DnsLookupResults{
        let mut results : Vec<DnsLookupResult> = vec![];
        for service in self.servicers {
            let svc_results = service.check(hostname);
            for r in svc_results.results {
                results.push(r)
            }
        } 
        let dlr = DnsLookupResults {results: results};
        return dlr;    
    }    
}