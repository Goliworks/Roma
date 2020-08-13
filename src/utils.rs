pub fn host_to_domain(host: String) -> String {
    let domain: Vec<&str> = host.split(":").collect();
    domain[0].to_string()
}
