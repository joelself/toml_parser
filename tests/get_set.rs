extern crate tomllib;
extern crate env_logger;
use tomllib::parser::TOMLParser;
use tomllib::types::ParseResult;

#[test]
fn test_mixed_tables() {
  let _ = env_logger::init();
	let input = r#"fish = "halibut"
[[foo."bar"]]
baz = 12345
qux = 1981-04-15
[[foo."bar"]]
baz = 'something'
qux = '''other'''
[[foo.quality]]
furniture = "chair"
[foo.quality.machine.parts.service]
"ƥèřïôδ" = 24.7
[[foo.quality.labor]]
Name = 'Rïçλářδ'
[[foo.quality.labor]]
Name = '§ƭèřℓïñϱ Âřçλèř'
[[foo.quality]]
money = 789.0123
[[foo."bar"]]
baz = 2016-03-10T12:31:02+07:30
qux = """ƒáβúℓôúƨ δïñôƨáúř"""
[foo]
"δïáϱñôƨïƨ" = true
"ƥřôϱñôƨïƨ" = "not good"
hypnosis = 987654321
"#;

  let parser = TOMLParser::new();
  let (_, result) = parser.parse(input);
  match result {
    ParseResult::FullError | ParseResult::Partial(_,_,_) |
    ParseResult::PartialError(_,_,_) | ParseResult::Failure(_,_) => assert!(false),
    _ => (),
  }
}