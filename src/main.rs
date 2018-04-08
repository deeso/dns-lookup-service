#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// extern crate toml;

extern crate trust_dns;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate iron;
use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

extern crate toml;


mod config;
mod lookup;
mod webserver;



fn main() {

    let mut is_server = false;
    let dsa: config::DnsServiceArgs = config::parse_args();

    let odscs = config::DnsServerConfigs::from_service_args(&dsa);
    match odscs.as_ref() {
        Some(dscs) => is_server = dscs.is_server.clone(),
        None => {
            println!("Failed to parse a dns server config from arguments");
            return;
        }
    }


    let dscs = odscs.unwrap();
    if is_server {
        webserver::run_server(&dscs)
    } else {

        let odlss = lookup::DnsLookupServices::from_service_configs(&dscs);
        
        match odlss {
            Some(_) => {}
            None => {
                println!("Failed to parse a dns server config from arguments");
                return;
            }
        }

        let sc = odlss.unwrap();
        let response = sc.check(&dsa.hostname);
        let j = serde_json::to_string(&response);
        println!("The results is:\n{}", j.unwrap());
        return;
    }
}
