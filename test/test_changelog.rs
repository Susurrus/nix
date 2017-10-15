extern crate pulldown_cmark;

use std::fs::File;
use std::io::Read;

use self::pulldown_cmark::Parser;

#[test]
fn test_changelog() {
    let mut f = File::open("CHANGELOG.md").unwrap();
    let mut changelog_text = String::new();
    f.read_to_string(&mut changelog_text).unwrap();
    let changelog_parser = Parser::new(&changelog_text);
    for e in changelog_parser {
        println!("{:?}", e);
    }

}
