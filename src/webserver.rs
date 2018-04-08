extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

use config;
use lookup;

static LOOKUP_REQ: &'static str = "/lookup";
static GET_REQ: &'static str = "/get";
static DEFAULT_REQ: &'static str = "/";


static TEST_MSG: &'static str = r#"{
                "test": "test test"
              }"#;



fn handle_lookup(request: &mut Request, config: &lookup::DnsLookupServices) -> IronResult<Response> {
    let mut payload : String = "{\"invalid\":true}".to_string();
    request.body.read_to_string(&mut payload);
    Ok(Response::with((status::Ok, payload)))
}

fn handle_get(request: &mut Request, config: &lookup::DnsLookupServices) -> IronResult<Response> {
    let mut payload : String = "{\"invalid\":true}".to_string();
    request.body.read_to_string(&mut payload);
    Ok(Response::with((status::Ok, payload)))
}


pub fn run_server(server_configs: &config::DnsServerConfigs) {
    let odlss = lookup::DnsLookupServices::from_service_configs(&server_configs);
    match odlss {
        Some(_) => {}
        None => {
            println!("Failed to parse a dns server config from arguments");
            return;
        }
    }

    let sc = odlss.unwrap().clone();
    let sc1 = odlss.unwrap().clone();
    let sc2 = odlss.unwrap().clone();
    let mut router = Router::new();
    // router.post(LOOKUP_REQ, dummy_lookup, "lookup");
    router.post(LOOKUP_REQ, move |request: &mut Request| handle_lookup(request, &sc.clone()), "lookup");
    router.get(GET_REQ, move |request: &mut Request| handle_get(request, &sc1.clone()), "get_dns_servers");
    router.get(DEFAULT_REQ, move |request: &mut Request| handle_get(request, &sc2.clone()), "index");

    let hostname = &server_configs.listen_host;
    let port = &server_configs.listen_port;
    let address = format!("{}:{}", hostname, port);
    Iron::new(router).http(address).unwrap();

}