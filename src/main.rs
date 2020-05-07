use roxmltree::Document;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use strong_xml::{XmlRead, XmlWrite};
use walkdir::{DirEntry, WalkDir};

mod macro_structs;
use macro_structs::{Macros, Properties};
#[macro_use]
mod badname_macro;

struct Transforms {
    class: Class,
    vars: Vec<Transform>,
}
struct Transform {
    val: SelectedValue,
    scaler: f32,
}

enum Class {
    S,
    M,
    L,
    XL,
}
impl Class {
    fn path(&self) -> &'static str {
        match *self {
            Class::S => "D:/x4_extract_split/assets/units/size_s",
            Class::M => "D:/x4_extract_split/assets/units/size_m",
            Class::L => "D:/x4_extract_split/assets/units/size_l",
            Class::XL => "D:/x4_extract_split/assets/units/size_xl",
        }
    }
}
enum SelectedValue {
    Hull,
    Missile,
    Unit,
    CounterMeasure,
}

fn main() {
    let transforms = Transforms {
        class: Class::S,
        vars: vec![
            Transform {
                val: SelectedValue::Hull,
                scaler: 21.0,
            },
            Transform {
                val: SelectedValue::Missile,
                scaler: 11.0,
            },
        ],
    };
    run_walk_and_transform(&transforms);
}

fn run_walk_and_transform(transforms: &Transforms) {
    let walker = WalkDir::new(transforms.class.path()).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let unwrapped_entry = entry.expect("unwrap entry");
        let path = unwrapped_entry.path();
        if !path.is_dir() {
            parse(path, transforms)
        }
    }
}

fn parse(path: &Path, transforms: &Transforms) {
    if let Ok(original_string) = fs::read_to_string(path) {
        if let Ok(mut macro_struct) = Macros::from_str(&original_string) {
            //this would be the part where we transform

            let string = transform(&mut macro_struct, transforms);
            // println!("{:?}", path);
            let doc_orig = Document::parse(original_string.as_str()).expect("parse1");
            let doc_new = Document::parse(string.as_str()).expect("parse2");
            compare_docs(doc_orig, doc_new);
            write(string.clone(), path).expect("failed to write")
        }
    }
}

fn transform(macro_struct: &mut Macros, transforms: &Transforms) -> String {
    for trans in transforms.vars.iter() {
        match trans.val {
            SelectedValue::Hull => {
                // if let Some(c) = macro_struct.macrochild.properties.hull.as_mut() {
                //     if let Some(orig) = c.max.as_mut() {
                //         if let Ok(orig) = orig.parse::<f32>() {
                //             c.max.replace((orig * trans.scaler).to_string());
                //         }
                //     }
                // }
          
                 badname1!(hull, badname2!(max))

            // badname2!(max);
            // badname!(hull, max)
            }
            SelectedValue::Missile => {
                if let Some(c) = macro_struct.macrochild.properties.storage.as_mut() {
                    if let Some(orig) = c.missile.as_mut() {
                        if let Ok(orig) = orig.parse::<f32>() {
                            c.missile.replace((orig * trans.scaler).to_string());
                        }
                    }
                }
            }
            SelectedValue::Unit => {
                if let Some(c) = macro_struct.macrochild.properties.storage.as_mut() {
                    if let Some(orig) = c.unit.as_mut() {
                        if let Ok(orig) = orig.parse::<f32>() {
                            c.unit.replace((orig * trans.scaler).to_string());
                        }
                    }
                }
            }
            SelectedValue::CounterMeasure => {
                if let Some(c) = macro_struct.macrochild.properties.storage.as_mut() {
                    if let Some(orig) = c.countermeasure.as_mut() {
                        if let Ok(orig) = orig.parse::<f32>() {
                            c.countermeasure.replace((orig * trans.scaler).to_string());
                        }
                    }
                }
            }
        }
    }
    let new_string = macro_struct.to_string().expect("create xml");
    new_string
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
            // println!("missing node: {}", o_name)
        }
    }
    for o_name in o_node_attr_names.iter() {
        if !n_node_attr_names.contains(o_name) {
            // println!("missing attr: {}", o_name)
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




// match trans.val {
//             SelectedValue::Hull => {
//                 if let Some(a) = macro_struct.macrochild.as_mut() {
//                     if let Some(b) = a.properties.as_mut() {
//                         if let Some(c) = b.hull.as_mut() {
//                             if let Some(orig) = c.max.as_mut() {
//                                 let orig: f32 = orig.parse().expect("cast String to f32 failed");
//                                 c.max.replace((orig * trans.scaler).to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//             SelectedValue::Missile => {
//                 if let Some(a) = macro_struct.macrochild.as_mut() {
//                     if let Some(b) = a.properties.as_mut() {
//                         if let Some(c) = b.storage.as_mut() {
//                             if let Some(orig) = c.missile.as_mut() {
//                                 let orig: f32 = orig.parse().expect("cast String to f32 failed");
//                                 c.missile.replace((orig * trans.scaler).to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//             SelectedValue::Unit => {
//                 if let Some(a) = macro_struct.macrochild.as_mut() {
//                     if let Some(b) = a.properties.as_mut() {
//                         if let Some(c) = b.storage.as_mut() {
//                             if let Some(orig) = c.unit.as_mut() {
//                                 let orig: f32 = orig.parse().expect("cast String to f32 failed");
//                                 c.unit.replace((orig * trans.scaler).to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//             SelectedValue::CounterMeasure => {
//                 if let Some(a) = macro_struct.macrochild.as_mut() {
//                     if let Some(b) = a.properties.as_mut() {
//                         if let Some(c) = b.storage.as_mut() {
//                             if let Some(orig) = c.countermeasure.as_mut() {
//                                 let orig: f32 = orig.parse().expect("cast String to f32 failed");
//                                 c.countermeasure.replace((orig * trans.scaler).to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//         }
