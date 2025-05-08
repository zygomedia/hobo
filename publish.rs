//# execute = "*"

use execute::command as cmd;

fn main() {
	assert!(cmd!("cargo publish").current_dir("hobo_derive").status().unwrap().success());
	assert!(cmd!("cargo publish").current_dir("hobo_css_macros").status().unwrap().success());
	assert!(cmd!("cargo publish").current_dir("hobo_css").status().unwrap().success());
	assert!(cmd!("cargo publish").current_dir("hobo").status().unwrap().success());
}
