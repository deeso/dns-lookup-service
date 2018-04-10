#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate trust_dns;
extern crate argparse;
extern crate iron;
extern crate router;
extern crate toml;

#[macro_use]
extern crate log;
extern crate log4rs;

mod config;
mod lookup;
mod webserver;



fn main() {

    let dsa: config::DnsServiceArgs = config::parse_args();
    let odscs = config::DnsServerConfigs::from_service_args(&dsa);
    match odscs.as_ref() {
        Some(_) => {
            debug!("Completed creating create dns-lookup-service config");
        },
        None => {
            error!("Failed to parse and create dns-lookup-service config from arguments");
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
                println!("\n{}\n", j.unwrap());
                return;                
            }
        }
        None => {
            error!("Failed to parse a dns server config from arguments");
            return;
        }
    }
}
