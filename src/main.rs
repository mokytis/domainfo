use domainfo;


fn main() {
    let query = domainfo::detect_query_type(std::env::args().nth(1).expect("No query specified"));
    match query {
        domainfo::QueryType::IPAddr(q) => {
            println!("{}", domainfo::bgp_tools_query(String::from(q)));
        }
        domainfo::QueryType::DomainName(q) => {
            domainfo::dns_lookup(q);
        }
    };
}
