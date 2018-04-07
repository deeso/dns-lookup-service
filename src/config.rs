use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

extern crate toml;
use toml::Value as Toml;


extern crate serde_json;
use serde_json::Value as Json;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

static SERVERS : &'static str = "servers";
static PORT: &'static str = "--port";
static HOSTNAME: &'static str = "--hostname";
static DNS_SERVER: &'static str = "--dns_server";
static V6: &'static str = "--ip6";
static CONFIG: &'static str = "--config";



#[derive(Serialize, Deserialize, Debug)]
pub struct DnsServerConfig {
    name: String,
    nameserver: String,
    ip4: bool,
    ip6: bool,
    listen_port: String,
    listen_host: String,
    use_tls: bool, 
}

#[derive(Serialize, Debug)]
pub struct DnsServerConfigs {
    servers: Vec<DnsServerConfig>,
}

pub fn parse_args(config: &mut String, 
                  use_v6: &mut bool, 
                  dns_server: &mut String, 
                  hostname: &mut String, 
                  dns_port: &mut String) {
    let mut ap = ArgumentParser::new();
    ap.set_description("Lookup host using specified server");
    // ap.refer(&mut verbose)
    //     .add_option(&["-v", "--verbose"], StoreTrue,
    //     "Be verbose");

    ap.refer(config)
        .add_option(&[CONFIG], Store,
        "use config");

    ap.refer(use_v6)
        .add_option(&[V6], StoreTrue,
        "use IPv6");

    ap.refer(dns_server)
        .add_option(&[DNS_SERVER], Store,
        "dns server to use");

    ap.refer(hostname)
        .add_option(&[HOSTNAME], Store,
        "hostname to resolve");

    ap.refer(dns_port)
        .add_option(&[PORT], Store,
        "port to use for the query");

    ap.parse_args_or_exit();

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

pub fn read_servers_config(name: String) -> Option<DnsServerConfigs> {
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
                                println!("Could not read the result: {}\nErr: {}", ds_str, err)
                            }
                        }                        
                    }
                    return Some(DnsServerConfigs{servers: results});
                }
                &None => return None

            }
        }
        None => println!("Could not read the result"),
    }
    return None;
}

pub fn read_config(name: String) -> Option<Json> {
    let mut input = String::new();
    let _res = File::open(&name).and_then(|mut f| {
        f.read_to_string(&mut input)
    });

    match _res {
        Ok(_ignore) => {

        }
        Err(error) => {
            println!("IO failed to open file: {}", error);  
            return None;            
        } 
    }
    
    match input.parse() {
        Ok(toml) => {
            let json = convert(toml);
            return Some(json);
        }
        Err(error) => println!("failed to parse TOML: {}", error),
    }
    return None;
}