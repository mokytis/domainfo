use domainfo;

fn main() {
    match std::env::args().nth(1) {
        Some(query) => {
            match domainfo::detect_query_type(query) {
                domainfo::QueryType::IPAddr(q) => {
                    print!("{}", domainfo::bgp_tools_query(&q));
                }
                domainfo::QueryType::DomainName(q) => {
                    print!("{}", domainfo::dns_lookup(&q));
                }
            };
        }
        None => {
            eprintln!(
                "Please provide a domain name or IP address\nUsage: domainfo <ipaddr or domain>"
            );
        }
    };
}
