use std::fs;
use std::fs::File;
extern crate tomllib;
use tomllib::parser::TOMLParser;
use std::io::{Read, BufReader};

fn verify(input: String) -> (bool, Option<(String, String)>) {
    let input_copy = input.clone();
    let parser = TOMLParser::new();
    let parser = parser.parse(&input_copy);
    let result = format!("{}", parser);
    if result != input  {
        return (false, Some((input, result)));
    } else {
        return (true, None);
    }
}

#[test]
fn test_all_assets() {
	let paths = fs::read_dir("./assets/valid/").unwrap();
    let mut failed: Vec<(String, Option<(String, String)>)> = vec![];

    for path in paths {
        let dir_entry = path.unwrap(); // DirEntry
        if dir_entry.file_type().unwrap().is_file() {
            let filename_osstr = dir_entry.file_name();
            let filename = filename_osstr.to_str().
                            unwrap_or_else(|| "").to_string();
            if filename == "" {
                failed.push((format!("{:?}", filename_osstr), None));
                continue;
            }
            let pathbuf = dir_entry.path();
            let filepath = pathbuf.to_str().unwrap_or_else(|| {failed.push((filename.clone(), None)); ""});
            if filepath == "" {
                continue;
            }
            let file_res = File::open(filepath);
            if !file_res.is_ok() {
                failed.push((filename.clone(), None));
                continue;
            }
            let file = file_res.unwrap();
            let mut contents = BufReader::new(&file);
            let mut buffer = String::new();
            if contents.read_to_string(&mut buffer).is_ok() {
                let (success, in_out) = verify(buffer);
                if !success {
                    failed.push((filename.clone(), in_out));
                }
            }
            else {
                failed.push((filename.clone(), None));
            }
        }
    }
    if failed.len() > 0 {
        let mut panic_string = String::new();
        for &(ref filename, ref in_out) in &failed {
            let s;
            match in_out {
                &Some((ref input, ref output)) =>
                {s = format!("Failed to correctly parse file \"{}\".\nExpected:\n\"{}\"\nGot:\n\"{}\"", filename, input, output)},
                &None => {s = format!("Failed to correctly parse file \"{}\"",filename)}
            }
            panic_string.push_str(&s);
        }
        assert!(false, panic_string);
    }
}