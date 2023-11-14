use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct SubdomainList {
    error_root: String,
    servers: Vec<Subdomain>,
    #[serde(rename = "static")]
    static_servers: Vec<StaticServe>,
}

#[derive(Deserialize, Debug)]
pub struct Subdomain {
    name: String,
    port: u16,
    disabled: Option<bool>,
    host: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct StaticServe {
    name: String,
    path: String,
    disabled: Option<bool>,
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

    // Write subdomain servers
    for subdomain in data.servers {
        // If not disabled, continue
        if !convert_optional_bool(subdomain.disabled) {
            output += generate_virtual_server(
                subdomain.name,
                subdomain.port,
                &data.error_root,
                convert_optional_bool(subdomain.host),
            )
            .as_str();
        }
    }

    // Write subdomain static servers
    for subdomain in data.static_servers {
        // If not disabled, continue
        if !convert_optional_bool(subdomain.disabled) {
            output += generate_static_server(subdomain.name, subdomain.path).as_str();
        }
    }

    // Write file
    fs::write("subdomains.conf", output).unwrap();
}

fn generate_virtual_server(name: String, port: u16, error_root: &String, has_host: bool) -> String {
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
    result += "\n";
    result += "\terror_page 502 /502.html;\n";
    result += "\tlocation /502.html {\n";
    result += format!("\t\troot {error_root};\n").as_str();
    result += "\t}\n";
    result += "}\n";
    result += "\n";

    result
}

fn generate_static_server(name: String, path: String) -> String {
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
    result += format!("\troot {path};\n").as_str();
    result += "\tindex index.html;\n";
    result += "\terror_page 404 /404.html;\n";
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
