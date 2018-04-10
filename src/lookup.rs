extern crate serde;
extern crate serde_json;

extern crate trust_dns;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::Message;
use trust_dns::rr::{DNSClass, Name, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use config;

#[derive(Debug, Serialize, Clone)]
pub struct DnsLookupService {
    pub name: String,
    pub dns_server: String,
    pub dns_port: String,
    pub use_tls: bool,
    pub ip4: bool,
    pub ip6: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsLookupServices {
    pub servicers: Vec<DnsLookupService>,
    pub listen_port: String,
    pub listen_host: String,
}

#[derive(Serialize, Debug)]
pub struct DnsLookupRequest {
    pub hostname: String,
}

impl DnsLookupRequest {
    // pub fn from_web_json(req_payload: &String) -> Option<DnsLookupRequest> {
    //     return None;
    // }

    pub fn from_key(hostname: &String) -> Option<DnsLookupRequest> {
        return Some(DnsLookupRequest{hostname: hostname.clone()});    
    }    

}

#[derive(Serialize, Debug)]
pub struct DnsLookupResult {
    source: String,
    name: String,
    address: String,
    atype: String,
    time_ms: u64,
}

#[derive(Serialize, Debug)]
pub struct DnsLookupResults {
    results: Vec<DnsLookupResult>,
}

pub struct DnsServerResponse {
    msg: Message,
    start_time: u64,
    end_time: u64,
}

fn extract_results(dsr: &DnsServerResponse, source: &String) -> DnsLookupResults {
    // let mut results = Vec<DnsLookupResult>::new();
    let mut results : Vec<DnsLookupResult> = vec![];

    let answers: &[Record] = dsr.msg.answers();
    let time_ms = &dsr.end_time - &dsr.start_time;
    if answers.len() > 0 {
        for ans in answers {
            let rdata = ans.rdata();
            if let RecordType::A = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                // println!("{} {}", ans.name(), (ip).to_string());
                let result = DnsLookupResult {
                    source: source.clone(),
                    atype: "ip4".to_string(),
                    name: ans.name().to_string().clone(),
                    address: (ip).to_string().clone(),
                    time_ms: time_ms,
                };
                results.push(result);

            } else if let RecordType::AAAA = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                let result = DnsLookupResult {
                    source: source.clone(),
                    atype: "ip6".to_string(),
                    name: ans.name().to_string().clone(),
                    address: (ip).to_string().clone(),
                    time_ms: time_ms,
                };
                results.push(result);
                // println!("{} {}", ans.name(), (ip).to_string());                      
            } 
        }
    }
    let r = DnsLookupResults {results: results};
    return r;
}

impl DnsLookupService {

    // pub fn from_service_config(dsc: &config::DnsServerConfig) -> Option<DnsLookupService> {
    //     return Some(DnsLookupService {
    //         dns_server: dsc.nameserver.clone(),
    //         dns_port: "53".to_string(),
    //         use_tls: false,
    //         ip4: dsc.ip4.clone(),
    //         ip6: dsc.ip6.clone(),
    //     })
    // }

    pub fn check(&self, hostname: &String) -> DnsLookupResults{
        let mut results : Vec<DnsLookupResult> = vec![];
        if self.ip4 {
            let response = self.check_ip4(&hostname);
            let dns_results = extract_results(&response, &self.name);
            let msg = format!("Extracted A {} records using {} for {}", dns_results.results.len(), &self.dns_server, &hostname);
            debug!("{}", msg);
            for r in dns_results.results {
                results.push(r)
            }
        } 
        if self.ip6 {
            let response = self.check_ip6(&hostname);
            let dns_results = extract_results(&response, &self.name);
            let msg = format!("Extracted AAAA {} records using {} for {}", dns_results.results.len(), &self.dns_server, &hostname);
            debug!("{}", msg);
            for r in dns_results.results {
                results.push(r)
            }
        }
        let dlr = DnsLookupResults {results: results};
        return dlr;    
    }

    fn check_ip4(&self, hostname: &String) -> DnsServerResponse{
        let name = Name::from_str(*&hostname).unwrap();
        let address = format!("{}:{}", self.dns_server, self.dns_port).parse().unwrap();
        let msg = format!("Looking up A using {:?} for {}", &address, &hostname);
        debug!("{}", msg);
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let start = SystemTime::now();
        let mut now_epoch = start.duration_since(UNIX_EPOCH);
        let mut start_time : u64 = 0;
        match now_epoch {
            Ok(result) => {
                start_time = result.as_secs() * 1000 + result.subsec_nanos() as u64 / 1_000_000;
            },
            Err(_) => {}
        }        
        let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
        let end = SystemTime::now();
        now_epoch = end.duration_since(UNIX_EPOCH);
        let mut end_time : u64 = 0;
        match now_epoch {
            Ok(result) => {
                end_time = result.as_secs() * 1000 + result.subsec_nanos() as u64 / 1_000_000;
            },
            Err(_) => {}
        }        

        let x = DnsServerResponse{
            msg: response,
            start_time: start_time,
            end_time: end_time,
        };
        return x;

    }

    fn check_ip6(&self, hostname: &String) -> DnsServerResponse{

        let name = Name::from_str(&*hostname).unwrap();
        let address = format!("{}:{}", self.dns_server, self.dns_port).parse().unwrap();
        let msg = format!("Looking up AAAA using {:?} for {}", &address, &hostname);
        debug!("{}", msg);
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let start = SystemTime::now();
        let mut now_epoch = start.duration_since(UNIX_EPOCH);
        let mut start_time : u64 = 0;
        match now_epoch {
            Ok(result) => {
                start_time = result.as_secs() * 1000 + result.subsec_nanos() as u64 / 1_000_000;
            },
            Err(_) => {}
        }        

        let response: Message = client.query(&name, DNSClass::IN, RecordType::AAAA).unwrap();
        let end = SystemTime::now();
        now_epoch = end.duration_since(UNIX_EPOCH);
        let mut end_time : u64 = 0;
        match now_epoch {
            Ok(result) => {
                end_time = result.as_secs() * 1000 + result.subsec_nanos() as u64 / 1_000_000;
            },
            Err(_) => {}
        }        

        let x = DnsServerResponse{
            msg: response,
            start_time: start_time,
            end_time: end_time,
        };
        return x;
    }

}

impl DnsLookupServices {
    pub fn from_service_configs(dsc: &config::DnsServerConfigs) -> Option<DnsLookupServices> {
        let mut results : Vec<DnsLookupService> = vec![];
        let servers = &dsc.servers;
        for c in servers {
            let msg = format!("Configuring {:?} using server: {} for ip4:{} and ip6:{}", &c.name, &c.nameserver, &c.ip4, &c.ip6);
            debug!("{}", msg);
            let dls = DnsLookupService {
                name: c.name.clone(),
                dns_server: c.nameserver.clone(),
                dns_port: "53".to_string(),
                use_tls: false,
                ip4: c.ip4.clone(),
                ip6: c.ip6.clone(),
            };
            results.push(dls);
        }
        return Some(DnsLookupServices{
            servicers:results,
            listen_port: dsc.listen_port.clone(),
            listen_host: dsc.listen_host.clone(),
        })
    }

    pub fn check(&self, hostname: &String) -> DnsLookupResults{
        let mut results : Vec<DnsLookupResult> = vec![];
        let servicers = &self.servicers; 
        for service in servicers {
            let msg = format!("Checking {:?} using server: {} for: {}", &service.name, &service.dns_server, hostname);
            debug!("{}", msg);

            let svc_results = service.check(hostname);
            for r in svc_results.results {
                results.push(r)
            }
        } 
        let dlr = DnsLookupResults {results: results};
        return dlr;    
    }    
}
