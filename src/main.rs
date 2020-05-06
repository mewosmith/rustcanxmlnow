use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use strong_xml::{XmlRead, XmlWrite};
use walkdir::{DirEntry, WalkDir};

mod macro_structs;
use macro_structs::Macros;
use roxmltree::Document;
fn main() {

    run_walk_and_transform("D:/x4_extract_split/assets/units/size_s");
    run_walk_and_transform("D:/x4_extract_split/assets/units/size_m");
    run_walk_and_transform("D:/x4_extract_split/assets/units/size_l");
    run_walk_and_transform("D:/x4_extract_split/assets/units/size_xl");
}
fn run_walk_and_transform(path: &str) {
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let unwrapped_entry = entry.expect("unwrap entry");
        let path = unwrapped_entry.path();
        if !path.is_dir() {
            parse(path)
        }
    }
}

fn parse(path: &Path) {
    if let Ok(original_string) = fs::read_to_string(path) {
        let macro_struct = Macros::from_str(&original_string);
        if let Ok(x) = macro_struct {
            let new_string = x.to_string().expect("create xml");
            let doc_orig = Document::parse(original_string.as_str()).expect("parse1");
            let doc_new = Document::parse(new_string.as_str()).expect("parse2");
            println!("{:?}", path);
            // println!("{:#?}", original_string);
            // println!("{:#?}", new_string);
            compare_docs(doc_orig, doc_new);
            write(new_string, path).expect("failed to write")
        }
    }
}

fn compare_docs(doc_orig: Document, doc_new: Document) {
    // collect the node names and the attr names into lists and compare the lists to see if one contains the other
    let mut o_node_names: Vec<String> = vec![];
    let mut o_node_attr_names: Vec<String> = vec![];
    let mut n_node_names: Vec<String> = vec![];
    let mut n_node_attr_names: Vec<String> = vec![];
    for o_node in doc_orig.descendants() {
        o_node_names.push(o_node.tag_name().name().to_owned());
        for o_attr in o_node.attributes() {
            o_node_attr_names.push(o_attr.name().to_owned())
        }
    }
    for n_node in doc_new.descendants() {
        n_node_names.push(n_node.tag_name().name().to_owned());
        for n_attr in n_node.attributes() {
            n_node_attr_names.push(n_attr.name().to_owned())
        }
    }
    for o_name in o_node_names.iter() {
        if !n_node_names.contains(o_name) {
            println!("missing node: {}", o_name)
        }
    }
    for o_name in o_node_attr_names.iter() {
        if !n_node_attr_names.contains(o_name) {
            println!("missing attr: {}", o_name)
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn write(s: String, path: &Path) -> std::io::Result<()> {
    let out_path = path.strip_prefix("D:/x4_extract_split/assets/").unwrap();
    let final_out = Path::new("X:/Rust Projects/round20ofwillitparse/testresults").join(out_path);
    let parent = final_out.parent().unwrap();
    fs::create_dir_all(&parent).expect("here");
    let mut file = File::create(final_out)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
