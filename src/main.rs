#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate trust_dns;
extern crate argparse;
// extern crate toml;


use trust_dns::client::{Client, SyncClient};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use trust_dns::op::Message;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;
use argparse::{ArgumentParser, StoreTrue, Store};

static HOSTNAME: &'static str = "--hostname";
static DNS_SERVER: &'static str = "--dns_server";
static V6: &'static str = "--ip6";

static CONFIG: &'static str = "--config";

#[derive(Serialize, Debug)]
struct QResult {
    name: String,
    address: String,
    atype: String,
}

#[derive(Serialize, Debug)]
struct QResponse<'b> {
    results: &'b mut Vec<QResult>
}


fn check_ip4(name: &Name, dns_server: &String, dns_port: &String ) -> Message{
    let address = format!("{}:{}", dns_server, dns_port).parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    return response;

}
fn check_ip6(name: &Name, dns_server: &String, dns_port: &String ) -> Message{
    let address = format!("{}:{}", dns_server, dns_port).parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let response: Message = client.query(&name, DNSClass::IN, RecordType::AAAA).unwrap();
    return response;
}

fn main() {

    //let mut verbose = false;
    let mut dns_server = "8.8.8.8".to_string();
    let mut hostname = "".to_string();
    let dns_port = "53".to_string();
    let mut use_v6 = false;

    
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Lookup host using specified server");
        // ap.refer(&mut verbose)
        //     .add_option(&["-v", "--verbose"], StoreTrue,
        //     "Be verbose");

        ap.refer(&mut use_v6)
            .add_option(&[V6], StoreTrue,
            "use IPv6");

        ap.refer(&mut dns_server)
            .add_option(&[DNS_SERVER], Store,
            "dns server to use");

        ap.refer(&mut hostname)
            .add_option(&[HOSTNAME], Store,
            "hostname to resolve");
    
        ap.parse_args_or_exit();
    }


    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(&*hostname).unwrap();

    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    // let response: Message = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    let response;
    if !use_v6 {
        response = check_ip4(&name, &dns_server, &dns_port);
    } else {
        response = check_ip6(&name, &dns_server, &dns_port);
    }
    

    // Messages are the packets sent between client and server in DNS.
    //  there are many fields to a Message. It's beyond the scope of these examples
    //  to explain them. See trust_dns::op::message::Message for more details.
    //  generally we will be insterested in the Message::answers
    let data = &mut QResponse{results: &mut Vec::new()};

    let answers: &[Record] = response.answers();
    if answers.len() > 0 {
        for ans in answers {
            let rdata = ans.rdata();
            if let RecordType::A = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                println!("{} {}", ans.name(), (ip).to_string());
                let result = QResult {
                    atype: "ip4".to_string(),
                    name: ans.name().to_string().clone(),
                    address: (ip).to_string().clone(),
                };
                data.results.push(result);

            } else if let RecordType::AAAA = ans.rr_type() {
                let ip = rdata.to_ip_addr().unwrap();
                println!("{} {}", ans.name(), (ip).to_string());                      
            } 
            // else if let RecordType::CNAME = ans.rr_type() {
            //     let cname = RData::CNAME(rdata);
            //     println!("{}", (cname).to_string());                      
            // }
            // match ans.rr_type() {
            //     RecordType::A => {
            //         let &RData::A(ref ip) = ans.rdata();
            //         println!("{}", (ip).to_string());                      
            //     }
            //     RecordType::AAAA => {
            //         let &RData::AAAA(ref ipv6) = ans.rdata();
            //         println!("{}", (ipv6).to_string());                      
            //     }
            //     _ => {
            //     }
            // }
        }
    }
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
