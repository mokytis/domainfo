pub mod domainfo {
    pub enum QueryType {}
    pub struct IPAddress {}
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

pub struct IPAddress {
    asn: u32,
    ip: String,
    prefix: String,
    country: String,
    registry: String,
    as_name: String,
}

pub struct DomainName {
    domain: String,
    ipaddrs: Vec<IPAddress>,
}

impl fmt::Display for IPAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) is advertised by {} (AS{} {} {})\n",
            self.ip, self.prefix, self.as_name, self.asn, self.registry, self.country,
        )
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut prefix = "";
        let mut response = String::from("");
        for ipaddr in self.ipaddrs.iter() {
            response += &format!(
                "{}{} has IPAddr {}\n{}",
                prefix, self.domain, ipaddr.ip, ipaddr
            );
            prefix = "\n";
        }
        write!(f, "{}", response)
    }
}

pub fn dns_lookup(domain: &str) -> DomainName {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let name = domain.parse::<Name>().unwrap();
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    let answers: &[Record] = response.answers();
    let mut ipaddrs: Vec<IPAddress> = Vec::new();
    for ans in answers.iter() {
        if let &RData::A(ref ip) = ans.rdata() {
            ipaddrs.push(bgp_tools_query(&ip.to_string()));
        };
        if let &RData::AAAA(ref ip) = ans.rdata() {
            ipaddrs.push(bgp_tools_query(&ip.to_string()));
        };
    }

    DomainName {
        domain: String::from(domain),
        ipaddrs: ipaddrs,
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

pub fn bgp_tools_query(query: &str) -> IPAddress {
    let mut stream = TcpStream::connect("bgp.tools:43").unwrap();
    let mut buffer = [0; 1024];

    stream.write((query.to_owned() + "\n").as_bytes()).unwrap();
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

    IPAddress {
        asn: u32::from_str_radix(parts[0], 10).unwrap(),
        ip: String::from(parts[1]),
        prefix: String::from(parts[2]),
        country: String::from(parts[3]),
        registry: String::from(parts[4]),
        as_name: String::from(parts[6]),
    }
}
