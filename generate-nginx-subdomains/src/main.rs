use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct SubdomainList {
    servers: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
pub struct Subdomain {
    name: String,
    port: u16,
    disabled: Option<bool>,
    host: Option<bool>,
}

fn main() {
    let mut output = String::new();

    // Write header
    output += "##########################################\n";
    output += "# Subdomains (Auto-Generated via Script) #\n";
    output += "##########################################\n";
    output += "\n";

    // Read TOML
    let data = fs::read_to_string("subdomains.toml").expect("Requires \"subdomains.toml\"!");
    let data = toml::from_str::<SubdomainList>(&data).expect("Invalid TOML format.");

    // Write subdomain entries
    for subdomain in data.servers {
        // If not disabled, continue
        if !convert_optional_bool(subdomain.disabled) {
            output += generate_virtual_server(
                subdomain.name,
                subdomain.port,
                convert_optional_bool(subdomain.host),
            )
            .as_str();
        }
    }

    // Write file
    fs::write("subdomains.conf", output).unwrap();
}

fn generate_virtual_server(name: String, port: u16, has_host: bool) -> String {
    let mut result = String::new();

    // HTTP
    result += "server {\n";
    result += "\tlisten 80;\n";
    result += "\tlisten [::]:80;\n";
    result += format!("\tserver_name {name};\n").as_str();
    result += "\treturn 308 https://$host$request_uri;\n";
    result += "}\n";
    result += "\n";

    // HTTPS
    result += "server {\n";
    result += "\tlisten 443 ssl;\n";
    result += "\tlisten [::]:443 ssl;\n";
    result += format!("\tserver_name {name};\n").as_str();
    result += "\n";
    result += "\tlocation / {\n";
    result += format!("\t\tproxy_pass http://localhost:{port};\n").as_str();
    if has_host {
        result += "\t\tproxy_set_header Host $host;\n";
    }
    result += "\t}\n";
    result += "}\n";
    result += "\n";

    result
}

fn convert_optional_bool(flag: Option<bool>) -> bool {
    match flag {
        Some(flag) => flag,
        None => false,
    }
}
