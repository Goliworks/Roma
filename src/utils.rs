use dirs::home_dir;

pub fn host_to_domain(host: String) -> String {
    let domain: Vec<&str> = host.split(":").collect();
    domain[0].to_string()
}

pub fn resolve_path(path: String) -> String {
    let spl: Vec<&str> = path.split("~/").collect();
    if spl[0].is_empty() && spl.len() > 1 {
        return format!("{}/{}", home_dir().unwrap().to_str().unwrap(), spl[1])
    }
    path
}
