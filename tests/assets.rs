extern crate tomllib;
use std::fs;
use std::fs::DirEntry;

fn process_toml_file<'a>(file: DirEntry) -> Option<&'a str> {
	// TODO: Read file => parse file => write file => compare file to original
	return Option::None;
}

#[test]
fn test_all_assets() {
	let paths = fs::read_dir("./assets/").unwrap();
    let mut failed: Vec<&str> = vec![];

    for path in paths {
    	let file = path.unwrap();
    	if file.file_type().unwrap().is_file() {
    		let path = &*file.path();
    		match path.extension() {
    			Some(ext) if ext.to_str() == Some("toml") => {
    				match process_toml_file(file) {
    					Some(ref name) 	=> failed.push(name),
    					_	 			=> {}
    				};
    			},
    			_ => {} 
    		};
    	}
    }
    // TODO: if failed is non-empty assert false with all the names of failed files
}