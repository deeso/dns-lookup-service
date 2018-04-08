extern crate iron;
extern crate router;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

use config;
use lookup;

static LOOKUP_REQ: &'static str = "/lookup";
static GET_REQ: &'static str = "/get";
static DEFAULT_REQ: &'static str = "/";

static ERROR: &'static str = "{\"error\":true, \"message\":\"invalid web request\"}";
static UNPARSEABLE_ERROR: &'static str = "{\"error\":true, \"message\":\"unable to parse web request\"}";
static TEST_MSG: &'static str = r#"{
                "test": "test test"
              }"#;



fn handle_lookup(request: &mut Request, dlss: &lookup::DnsLookupServices) -> IronResult<Response> {
    let mut payload : String = "{\"invalid\":true}".to_string();
    let rrts = request.body.read_to_string(&mut payload);
    match rrts {
        Ok(_) => {},
        Err(_) => {
            return Ok(Response::with((status::Ok, ERROR.to_string())))
        } 
    }
    let odlwr = lookup::DnsLookupRequest::from_web_json(&payload);
    // match odlwr.as_ref() {
    //     Some(dlwr) => {
    //         let rjson_result = serde_json::to_string(&dlwr);
    //         match rjson_result.as_ref() {
    //             Ok(json_result) => {
    //                 return Ok(Response::with((status::Ok, json_result.clone())));
    //             },
    //             Err(_) => {
    //                 return Ok(Response::with((status::Ok, UNPARSEABLE_ERROR.to_string())));
    //             }

    //         }
    //     },
    //     None => {
    //         return Ok(Response::with((status::Ok, ERROR.to_string())));
    //     }
    // }

    let lookup_results = dlss.check(&odlwr.unwrap().name);
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

fn handle_get(_: &mut Request, dlss: &lookup::DnsLookupServices) -> IronResult<Response> {
 
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


pub fn run_server(dlss: &lookup::DnsLookupServices) {
    let sc = dlss.clone();
    let sc1 = dlss.clone();
    let sc2 = dlss.clone();
    let mut router = Router::new();
    // router.post(LOOKUP_REQ, dummy_lookup, "lookup");
    router.post(LOOKUP_REQ, move |request: &mut Request| handle_lookup(request, &sc.clone()), "lookup");
    router.get(GET_REQ, move |request: &mut Request| handle_get(request, &sc1.clone()), "get_dns_servers");
    router.get(DEFAULT_REQ, move |request: &mut Request| handle_get(request, &sc2.clone()), "index");

    let hostname = &dlss.listen_host;
    let port = &dlss.listen_port;
    let address = format!("{}:{}", hostname, port);
    Iron::new(router).http(address).unwrap();
}