extern crate tomllib;
//use tomllib::primitives::string;
use tomllib::toml::toml;
    //println!("Hello in English: {}", tomllib::ast::structs::hello());
fn main() {
    println!("{:?}", toml(r#"# Tλïƨ ïƨ á TÓM£ δôçú₥èñƭ.

title = "TÓM£ Éжá₥ƥℓè"

[owner]
name = "Tô₥ Þřèƨƭôñ-Wèřñèř"
dob = 1979-05-27T07:32:00-08:00 # Fïřƨƭ çℓáƨƨ δáƭèƨ

[database]
server = "192.168.1.1"
ports = [ 8001, 8001, 8002 ]
connection_max = 5000
enabled = true"#));

 }