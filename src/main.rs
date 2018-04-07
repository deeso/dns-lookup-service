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




fn main() {

    // fn lookup(_: &mut Request) -> IronResult<Response> {
    //     let response;
    //     if  config.atype == "ip4" {
    //         response = check_ip4(&name, &config);
    //     } else {
    //         response = check_ip6(&name, &config);
    //     }
    //     let payload = json::encode(&greeting).unwrap();
    //     Ok(Response::with((status::Ok, payload)))
    // }

    // fn get_config(_: &mut Request) -> IronResult<Response> {
    //     let payload = serde_json::to_string(&config);
    //     Ok(Response::with((status::Ok, payload)))
    // }

    // let mut router = Router::new();
    // router.get("/lookup", lookup);
    // router.get("/get", get_config);
    // router.post("/set", set_config);


    //let mut verbose = false;
    let mut dns_server = "8.8.8.8".to_string();
    let mut hostname = "".to_string();
    let mut dns_port = "53".to_string();
    let mut atype = "ip4".to_string();
    let mut config = "".to_string();
    let mut use_v6 = false;

    config::parse_args(&mut config,&mut use_v6,&mut dns_server,&mut hostname,&mut dns_port);


    if config.len() > 0 {
        let dsc = config::read_servers_config(config);
        let json_result = serde_json::to_string(&dsc);

        match json_result {
            // The division was valid
            Ok(json) => {
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            // The division was invalid
            Err(_)    => println!("Could not read the result"),
        }
    }

    if use_v6 {
        atype = "ip6".to_string();
    }


    // Specify the name, note the final '.' which specifies it's an FQDN

    let config = lookup::LookupConfig{
        dns_server: dns_server,
        dns_port: dns_port, 
        atype: atype, 
    };
    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    // let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    let response;
    if  config.atype == "ip4" {
        response = lookup::check_ip4(&hostname, &config);
    } else {
        response = lookup::check_ip6(&hostname, &config);
    }
    
    let data = lookup::extract_results(&response);
    // Messages are the packets sent between client and server in DNS.
    //  there are many fields to a Message. It's beyond the scope of these examples
    //  to explain them. See trust_dns::op::message::Message for more details.
    //  generally we will be insterested in the Message::answers
    let j = serde_json::to_string(&data);
    println!("The results is:\n{}", j.unwrap());
    //println!(answers[0].rdata()::);
    // Records are generic objects which can contain any data.
    //  In order to access it we need to first check what type of record it is
    //  In this case we are interested in A, IPv4 address
/*    if let &RData::A(ref ip) = answers[0].rdata() {
        println!("{}", (ip).to_string());
        
    } else {
        assert!(false, "unexpected result")
    }*/

}
