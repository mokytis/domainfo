pub mod domainfo {
    pub enum QueryType {}
    pub struct BGPToolsResponse {}
    pub fn dns_lookup() {}
    pub fn detect_query_type() {}
    pub fn bgp_tools_query() {}
}

use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

pub enum QueryType {
    DomainName(String),
    IPAddr(String),
}

pub struct BGPToolsResponse {
    asn: u32,
    ip: String,
    prefix: String,
    country: String,
    registry: String,
    as_name: String,
}

impl fmt::Display for BGPToolsResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) is advertised by {} (AS{} {} {})",
            self.ip, self.prefix, self.as_name, self.asn, self.registry, self.country,
        )
    }
}

pub fn dns_lookup(domain: String) {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let name = domain.parse::<Name>().unwrap();
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    let answers: &[Record] = response.answers();
    let mut prefix = "";
    for ans in answers.iter() {
        if let &RData::A(ref ip) = ans.rdata() {
            println!(
                "{}{} has IPv4 {}\n{}",
                prefix,
                domain,
                ip,
                bgp_tools_query(ip.to_string())
            );
            prefix = "\n";
        };
        if let &RData::AAAA(ref ip) = ans.rdata() {
            println!(
                "{}{} has IPv6 {}\n{}",
                prefix,
                domain,
                ip,
                bgp_tools_query(ip.to_string())
            );
            prefix = "\n";
        };
    }
}

pub fn detect_query_type(query: String) -> QueryType {
    let dot_count = query.matches(".").count();
    match dot_count {
        0 => QueryType::IPAddr(query),
        3 if query
            .split(".")
            .filter_map(|x| x.parse::<u32>().ok())
            .filter(|&x| x < 255)
            .count()
            == 4 =>
        {
            QueryType::IPAddr(query)
        }
        _ => QueryType::DomainName(query),
    }
}

pub fn bgp_tools_query(query: String) -> BGPToolsResponse {
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
