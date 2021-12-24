use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

enum QueryType {
    DomainName(String),
    IPAddr(String),
}

struct BGPToolsResponse {
    asn: u32,
    ip: String,
    prefix: String,
    country: String,
    registry: String,
    as_name: String,
}

impl BGPToolsResponse {
    fn output(&self) {
        println!(
            "{} ({}) is advertised by {} (AS{} {} {})",
            self.ip, self.prefix, self.as_name, self.asn, self.registry, self.country,
        );
    }
}

fn dns_lookup(domain: String) {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    let name = Name::from_str(&domain).unwrap();
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    let answers: &[Record] = response.answers();
    let mut prefix = "";
    for ans in answers.iter() {
        if let &RData::A(ref ip) = ans.rdata() {
            println!("{}{} has IPv4 {}", prefix, domain, ip);
            bgp_tools_query(ip.to_string()).output();
            prefix = "\n";
        };
        if let &RData::AAAA(ref ip) = ans.rdata() {
            println!("{}{} has IPv6 {}", prefix, domain, ip);
            bgp_tools_query(ip.to_string()).output();
            prefix = "\n";
        };
    }
}

fn detect_query_type(query: String) -> QueryType {
    let dot_count = query.matches(".").count();
    if dot_count >= 1 {
        // it at least 1 dot, count be IPv4 or domain name
        if dot_count == 3 {
            // has to have 3 dots to be ipv4
            for s in query.split(".") {
                match s.parse::<u32>() {
                    Ok(value) => {
                        if 255 < value {
                            return QueryType::DomainName(query);
                        }
                    }
                    Err(_) => {
                        return QueryType::DomainName(query);
                    }
                };
            }
            return QueryType::IPAddr(query);
        }
        return QueryType::DomainName(query);
    }
    // if no dots, assume IPv6 address
    QueryType::IPAddr(query)
}

fn bgp_tools_query(query: String) -> BGPToolsResponse {
    let mut stream = TcpStream::connect("bgp.tools:43").unwrap();
    let mut buffer = [0; 1024];

    stream.write((query + "\n").as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();

    let parts = std::str::from_utf8(&buffer)
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&part| part.trim())
        .collect::<Vec<&str>>();

    BGPToolsResponse {
        asn: u32::from_str_radix(parts[0], 10).unwrap(),
        ip: String::from(parts[1]),
        prefix: String::from(parts[2]),
        country: String::from(parts[3]),
        registry: String::from(parts[4]),
        as_name: String::from(parts[6]),
    }
}

fn main() {
    let query = detect_query_type(std::env::args().nth(1).expect("No query specified"));
    match query {
        QueryType::IPAddr(q) => {
            let response = bgp_tools_query(String::from(q));
            response.output();
        }
        QueryType::DomainName(q) => {
            dns_lookup(q);
        }
    };
}
