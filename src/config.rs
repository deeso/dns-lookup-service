use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

extern crate toml;
use toml::Value as Toml;


extern crate serde_json;
use serde_json::Value as Json;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate log4rs;

static SERVERS : &'static str = "servers";
static PORT: &'static str = "--port";
static HOSTNAME: &'static str = "--hostname";
static DNS_SERVER: &'static str = "--dns_server";
static V4: &'static str = "--ip4";
static V6: &'static str = "--ip6";
static CONFIG: &'static str = "--config";
static LOG_CONFIG: &'static str = "--log_config";
static SERVER: &'static str = "--iron_server";
static LHOST: &'static str = "--lhost";
static LPORT: &'static str = "--lport";



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsServerConfig {
    pub  name: String,
    pub  nameserver: String,
    pub  ip4: bool,
    pub  ip6: bool,
    // pub  use_tls: bool, 
}

pub struct DnsServiceArgs {
    pub  config: String,
    pub  log_config: String,
    pub  ip4: bool,
    pub  ip6: bool,
    pub  dns_server: String,
    pub  hostname: String,
    pub  port: String,
    pub  is_server: bool,
    pub  listen_port: String,
    pub  listen_host: String,
    // pub use_tls: bool,
}



#[derive(Serialize, Debug, Clone)]
pub struct DnsServerConfigs {
    pub servers: Vec<DnsServerConfig>,
    pub listen_port: String,
    pub listen_host: String,
    pub is_server: bool,
}

pub fn parse_args() -> DnsServiceArgs {
    let mut dns_server = "8.8.8.8".to_string();
    let mut dns_port = "53".to_string();
    let mut listen_host = "127.0.0.1".to_string();
    let mut listen_port = "8989".to_string();
    let mut hostname = "".to_string();
    // let mut atype = "ip4".to_string();
    let mut config_file = "".to_string();
    let mut ip6 = false;
    let mut ip4 = false;
    let mut is_server = false;
    let mut log_config_file = "".to_string();

    
    // must be scoped because of the mutable borrow
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Lookup host using specified server");
        // ap.refer(&mut verbose)
        //     .add_option(&["-v", "--verbose"], StoreTrue,
        //     "Be verbose");

        ap.refer(&mut config_file)
            .add_option(&[CONFIG], Store,
            "use config");

        ap.refer(&mut log_config_file)
            .add_option(&[LOG_CONFIG], Store,
            "configure the logger with");

        ap.refer(&mut ip6)
            .add_option(&[V6], StoreTrue,
            "use IPv6");

        ap.refer(&mut ip4)
            .add_option(&[V4], StoreTrue,
            "use IPv4");

        ap.refer(&mut is_server)
            .add_option(&[SERVER], StoreTrue,
            "is a server");

        ap.refer(&mut listen_port)
            .add_option(&[LPORT], Store,
            "listen port");

        ap.refer(&mut listen_host)
            .add_option(&[LHOST], Store,
            "listen server");

        ap.refer(&mut dns_server)
            .add_option(&[DNS_SERVER], Store,
            "dns server to use");

        ap.refer(&mut hostname)
            .add_option(&[HOSTNAME], Store,
            "hostname to resolve");

        ap.refer(&mut dns_port)
            .add_option(&[PORT], Store,
            "port to use for the query");

        ap.parse_args_or_exit();
    }


    if !ip4 && !ip6 {
        ip4 = true;
    }

    return DnsServiceArgs {
        listen_host: listen_host,
        listen_port: listen_port,
        config: config_file,
        log_config: log_config_file,
        ip6: ip6,
        ip4: ip4,
        dns_server: dns_server,
        hostname: hostname,
        port: dns_port,
        is_server: is_server,
    };
}


fn convert(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f)
                        .expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
        Toml::Table(table) => Json::Object(table.into_iter().map(|(k, v)| {
            (k, convert(v))
        }).collect()),
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

pub fn read_servers_config(name: &String) -> Option<Vec<DnsServerConfig>> {
    let read_result = read_config(name);

    match read_result {
        // The division was valid
        Some(json_result) => {
            let mut results : Vec<DnsServerConfig> = vec![];
            let o_servers = &json_result[SERVERS].as_object();
            match o_servers {
                &Some(servers) => {
                    for sc in servers.values() {
                        let ds_str = sc.to_string();
                        let r_sc = serde_json::from_value(sc.clone());
                        match r_sc {
                            Ok(result) => {
                                results.push(result);        
                            }
                            Err (err) => {
                                error!("Could not read the result: {}\nErr: {}", ds_str, err)
                            }
                        }                        
                    }
                    return Some(results);
                }
                &None => return None

            }
        }
        None => error!("Could not read the result"),
    }
    return None;
}

pub fn read_config(name: &String) -> Option<Json> {
    let mut input = String::new();
    let _res = File::open(name).and_then(|mut f| {
        f.read_to_string(&mut input)
    });

    match _res {
        Ok(_ignore) => {

        }
        Err(error) => {
            error!("IO failed to open file: {}", error);  
            return None;            
        } 
    }
    
    match input.parse() {
        Ok(toml) => {
            let json = convert(toml);
            return Some(json);
        }
        Err(error) => error!("failed to parse TOML: {}", error),
    }
    return None;
}


impl DnsServerConfigs {

    pub fn from_service_args(dsa: &DnsServiceArgs) -> Option<DnsServerConfigs> {

        let listen_host = dsa.listen_host.clone();
        let listen_port = dsa.listen_port.clone();
        let is_server = dsa.is_server.clone();
        let mut odscs : Option<DnsServerConfigs> = None;

        if dsa.log_config.len() > 0 {
            log4rs::init_file(&dsa.log_config, Default::default()).unwrap();
        }
        if dsa.config.len() > 0 {
            let o_servers = read_servers_config(&dsa.config);
            match o_servers {
                None => {}
                Some(servers) => {
                    let dscs = DnsServerConfigs {
                        servers: servers,
                        listen_port: listen_port,
                        listen_host: listen_host,
                        is_server: is_server,
                    };
                    // let json_result = serde_json::to_string(&dscs);
                    // match json_result {
                    //     // The division was valid
                    //     Ok(json) => {
                    //         println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    //     }
                    //     // The division was invalid
                    //     Err(_)    => println!("Could not read the result"),
                    // }
                    odscs = Some(dscs);
                }
            }
        } else {
            let dsc = DnsServerConfig {
                name: "command_line".to_string(),
                nameserver: dsa.dns_server.clone(),
                ip4: dsa.ip4.clone(),
                ip6: dsa.ip6.clone(),
            };

            let dscs = DnsServerConfigs {
                servers: vec![dsc],
                listen_port: listen_port,
                listen_host: listen_host,
                is_server: is_server,
            };
            odscs = Some(dscs);
            // let json_result = serde_json::to_string(&dscs);
            // match json_result {
            //     // The division was valid
            //     Ok(json) => {
            //         println!("{}", serde_json::to_string_pretty(&json).unwrap());
            //     }
            //     // The division was invalid
            //     Err(_)    => println!("Could not read the result"),
            // };

        }
        return odscs;
    }
}