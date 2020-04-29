use std::fs;
use std::fs::File;
use std::io::prelude::*;
use strong_xml::{XmlRead, XmlWrite};

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "macros")]
struct Macros {
    #[xml(child = "macro")]
    macrochild: Option<MacrosChild>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "macro")]
struct MacrosChild {
    #[xml(attr = "name")]
    name: Option<String>,
    #[xml(attr = "class")]
    class: Option<String>,
    #[xml(child = "component")]
    component: Option<Component>,
    #[xml(child = "properties")]
    properties: Option<Properties>,
    #[xml(child = "connections")]
    connections: Option<Connections>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "component")]
struct Component {
    #[xml(attr = "ref")]
    reference: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "properties")]
struct Properties {
    #[xml(child = "identification")]
    identification: Vec<Identification>,
    #[xml(child = "software")]
    software: Vec<Software>,
    #[xml(child = "explosiondamage")]
    explosion: Vec<Explosion>,
    #[xml(child = "storage")]
    storage: Vec<Storage>,
    #[xml(child = "hull")]
    hull: Vec<Hull>,
    #[xml(child = "secrecy")]
    secrecy: Vec<Secrecy>,
    #[xml(child = "purpose")]
    purpose: Vec<Purpose>,
    #[xml(child = "people")]
    people: Vec<People>,
    #[xml(child = "physics")]
    physics: Vec<Physics>,
    #[xml(child = "thruster")]
    thruster: Vec<Thruster>,
    #[xml(child = "ship")]
    ship: Vec<Ship>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "explosiondamage")]
struct Explosion {
    #[xml(attr = "value")]
    explosion: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "storage")]
struct Storage {
    #[xml(attr = "unit")]
    unit: Option<String>,
    #[xml(attr = "missile")]
    missile: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "hull")]
struct Hull {
    #[xml(attr = "max")]
    max: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "secrecy")]
struct Secrecy {
    #[xml(attr = "level")]
    level: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "purpose")]
struct Purpose {
    #[xml(attr = "primary")]
    primary: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "people")]
struct People {
    #[xml(attr = "capacity")]
    capacity: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "physics")]
struct Physics {
    #[xml(child = "inertia")]
    inertia: Option<Inertia>,
    #[xml(child = "drag")]
    drag: Option<Drag>,
    #[xml(attr = "mass")]
    mass: Option<String>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "inertia")]
struct Inertia {
    #[xml(attr = "pitch")]
    pitch: Option<String>,
    #[xml(attr = "yaw")]
    yaw: Option<String>,
    #[xml(attr = "roll")]
    roll: Option<String>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "drag")]
struct Drag {
    #[xml(attr = "forward")]
    forward: Option<String>,
    #[xml(attr = "reverse")]
    reverse: Option<String>,
    #[xml(attr = "horizontal")]
    horizontal: Option<String>,
    #[xml(attr = "vertical")]
    vertical: Option<String>,
    #[xml(attr = "pitch")]
    pitch: Option<String>,
    #[xml(attr = "yaw")]
    yaw: Option<String>,
    #[xml(attr = "roll")]
    roll: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "thruster")]
struct Thruster {
    #[xml(attr = "tags")]
    tags: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "ship")]
struct Ship {
    #[xml(attr = "type")]
    shiptype: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "software")]
struct Software {
    #[xml(child = "software")]
    software: Vec<SoftwareWares>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "software")]
struct SoftwareWares {
    #[xml(attr = "software")]
    software: Option<String>,
    #[xml(attr = "compatible")]
    compatible: Option<String>,
    #[xml(attr = "default")]
    default: Option<String>,
    #[xml(attr = "ware")]
    ware: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "identification")]
struct Identification {
    #[xml(attr = "name")]
    name: Option<String>,
    #[xml(attr = "basename")]
    basename: Option<String>,
    #[xml(attr = "description")]
    description: Option<String>,
    #[xml(attr = "variation")]
    variation: Option<String>,
    #[xml(attr = "shortvariation")]
    shortvariation: Option<String>,
    #[xml(attr = "icon")]
    icon: Option<String>,
    #[xml(attr = "unique")]
    unique: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "connections")]
struct Connections {
    #[xml(child = "connection")]
    connection: Vec<Connection>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "connection")]
struct Connection {
    #[xml(attr = "ref")]
    refer: Option<String>,
    #[xml(child = "macro")]
    macroref: Option<MacroRef>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "macro")]
struct MacroRef {
    #[xml(attr = "ref")]
    refer: Option<String>,
    #[xml(attr = "connection")]
    connection: Option<String>,
}
fn main() {
    let x = Macros::from_str(
        &fs::read_to_string(
            "D:/x4_extract_split/assets/units/size_xl/macros/ship_arg_xl_builder_01_a_macro.xml",
        )
        .unwrap(),
    )
    .unwrap();
    println!("{:#?}", x);

    let y = x.to_string().unwrap();
    println!("{:#?}", y);
    write(y).expect("failed to write")
}
fn write(s: String) -> std::io::Result<()> {
    let mut file = File::create("X:/Rust Projects/round20ofwillitparse/testresults/test.xml")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
