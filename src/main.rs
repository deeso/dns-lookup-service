#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate trust_dns;
extern crate argparse;
extern crate iron;
extern crate router;
extern crate toml;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;


mod config;
mod lookup;
mod webserver;



fn main() {

    let dsa: config::DnsServiceArgs = config::parse_args();
    println!("Failed to parse a dns server config from arguments");
    let odscs = config::DnsServerConfigs::from_service_args(&dsa);
    match odscs.as_ref() {
        Some(_) => {},
        None => {
            println!("Failed to parse a dns server config from arguments");
            return;
        }
    }

    let dscs = odscs.unwrap();
    let odlss = lookup::DnsLookupServices::from_service_configs(&dscs);    
    match odlss.as_ref() {
        Some(dlss) => {
            if dscs.is_server {
                webserver::run_iron_server(dlss)
            } else {
                let response = dlss.check(&dsa.hostname);
                let j = serde_json::to_string(&response);
                println!("The results is:\n{}", j.unwrap());
                return;                
            }
        }
        None => {
            println!("Failed to parse a dns server config from arguments");
            return;
        }
    }
}
