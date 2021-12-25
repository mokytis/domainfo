use domainfo;

fn main() {
    let query = domainfo::detect_query_type(std::env::args().nth(1).expect("No query specified"));
    match query {
        domainfo::QueryType::IPAddr(q) => {
            print!("{}", domainfo::bgp_tools_query(&q));
        }
        domainfo::QueryType::DomainName(q) => {
            print!("{}", domainfo::dns_lookup(&q));
        }
    };
}
