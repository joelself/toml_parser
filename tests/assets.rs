use std::fs;
use std::fs::{DirEntry,File};
extern crate tomllib;
use tomllib::parser::Parser;
use std::io::Read;

fn process_toml_file<'a>(entry: DirEntry) -> Option<&'a str> {
	// TODO: Read file => parse file => write file => compare file to original
    let path = entry.path();
    let path_str_opt = path.to_str();
    let path_str = path_str_opt.expect("Unable to convert file PathBuffer to string.");
    let file_res: Result<File, _> = File::open(path_str);
    let file_ok : Option<File> = file_res.ok();
    let mut f: File = file_ok.expect("Was not able to get file from path");
	let mut file: File = try!(Ok(f));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let parser = Parser::new();
    parser.parse(&contents[..]);
    println!("{}", parser);
    Some("Remove me")
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
            break; // Just see if this works
    	}
    }
    // TODO: if failed is non-empty assert false with all the names of failed files
}