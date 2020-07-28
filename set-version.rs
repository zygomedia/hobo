//# regex = "*"

use regex::Regex;

fn main() {
	let new_version = std::env::args().skip(1).next().unwrap();
	for manifest in ["lib/Cargo.toml", "css/Cargo.toml", "css/macros_decl/Cargo.toml"] {
		let mut x = std::fs::read_to_string(manifest).unwrap();
		x = Regex::new(r#"hobo_css = "=[0-9\.]+""#            ).unwrap().replace_all(x, format!(r#"hobo_css = "={}""#,             new_version));
		x = Regex::new(r#"hobo_css_macros = "=[0-9\.]+""#     ).unwrap().replace_all(x, format!(r#"hobo_css_macros = "={}""#,      new_version));
		x = Regex::new(r#"hobo_css_macros_decl = "=[0-9\.]+""#).unwrap().replace_all(x, format!(r#"hobo_css_macros_decl = "={}""#, new_version));
		std::fs::write(manifest, x).unwrap();
	}
}
