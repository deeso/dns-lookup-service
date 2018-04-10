extern crate iron;
extern crate router;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

use lookup;

static LOOKUP_REQ: &'static str = "/lookup/:hostname";
static DEFAULT_REQ: &'static str = "/:hostname";
static GET_REQ: &'static str = "/get_config";

static ERROR: &'static str = "{\"error\":true, \"message\":\"invalid web request\"}";
static UNPARSEABLE_ERROR: &'static str = "{\"error\":true, \"message\":\"unable to parse web request\"}";


fn handle_lookup(request: &mut Request, dlss: &lookup::DnsLookupServices) -> IronResult<Response> {
    let mut payload : String = "{\"invalid\":true}".to_string();
    let ref hostname = request.extensions.get::<Router>().unwrap().find("hostname").unwrap_or("/");
    let rrts = request.body.read_to_string(&mut payload);
    let peer = &request.remote_addr;
    let msg = format!("Handling dns-lookup request from {:?} for {}", peer, &hostname);
    info!("{}", msg);
    match rrts {
        Ok(_) => {},
        Err(_) => {
            return Ok(Response::with((status::Ok, ERROR.to_string())))
        } 
    }
    let odlwr = lookup::DnsLookupRequest::from_key(&hostname.to_string());
    let lookup_results = dlss.check(&odlwr.unwrap().hostname);
    let rjson_result = serde_json::to_string(&lookup_results);
    match rjson_result.as_ref() {
        Ok(json_result) => {
            return Ok(Response::with((status::Ok, json_result.clone())));
        },
        Err(_) => {
            return Ok(Response::with((status::Ok, UNPARSEABLE_ERROR.to_string())));
        }

    }
}

fn handle_get(request: &mut Request, dlss: &lookup::DnsLookupServices) -> IronResult<Response> {
    let peer = &request.remote_addr;
    let msg = format!("Handling dns-lookup-get request from {:?}", peer);
    info!("{}", msg);
 
    let rjson_result = serde_json::to_string(dlss);
    match rjson_result.as_ref() {
        Ok(json_result) => {
            return Ok(Response::with((status::Ok, json_result.clone())));
        },
        Err(_) => {
            return Ok(Response::with((status::Ok, UNPARSEABLE_ERROR.to_string())));
        }

    }
}

pub fn run_iron_server(dlss: &lookup::DnsLookupServices) {
    let sc = dlss.clone();
    let sc1 = dlss.clone();
    let sc2 = dlss.clone();
    let mut router = Router::new();
    // router.post(LOOKUP_REQ, dummy_lookup, "lookup");
    router.get(LOOKUP_REQ, move |request: &mut Request| handle_lookup(request, &sc.clone()), "lookup");
    router.get(DEFAULT_REQ, move |request: &mut Request| handle_lookup(request, &sc2.clone()), "index");
    router.get(GET_REQ, move |request: &mut Request| handle_get(request, &sc1.clone()), "get_dns_servers");

    let hostname = &dlss.listen_host;
    let port = &dlss.listen_port;
    let msg = format!("Starting to listen for dns-lookup requests on {}:{}", &hostname, &port);
    info!("{}", msg);
    let address = format!("{}:{}", hostname, port);
    Iron::new(router).http(address).unwrap();
}