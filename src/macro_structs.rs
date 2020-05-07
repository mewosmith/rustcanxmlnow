use strong_xml::{XmlRead, XmlWrite};

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "macros")]
pub struct Macros {
    #[xml(child = "macro")]
    pub macrochild: MacrosChild,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "macro")]
pub struct MacrosChild {
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "class")]
    pub class: Option<String>,
    #[xml(child = "component")]
    pub component: Option<Component>,
    #[xml(child = "properties")]
    pub properties: Properties,
    #[xml(child = "connections")]
    pub connections: Option<Connections>,
     #[xml(attr = "alias")]
    pub alias: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "component")]
pub struct Component {
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "properties")]
pub struct Properties {
    #[xml(child = "identification")]
    pub identification: Vec<Identification>,
    #[xml(child = "software")]
    pub software: Vec<Software>,
    #[xml(child = "explosiondamage")]
    pub explosion: Vec<ExplosionDam>,
    #[xml(child = "storage")]
    pub storage: Option<Storage>,
    #[xml(child = "hull")]
    pub hull: Option<Hull>,
    #[xml(child = "secrecy")]
    pub secrecy: Option<Secrecy>,
    #[xml(child = "purpose")]
    pub purpose: Option<Purpose>,
    #[xml(child = "people")]
    pub people: Option<People>,
    #[xml(child = "physics")]
    pub physics: Option<Physics>,
    #[xml(child = "thruster")]
    pub thruster: Option<Thruster>,
    #[xml(child = "ship")]
    pub ship: Option<Ship>,
    #[xml(child = "loadouts")]
    pub loadouts: Option<Loadouts>,
    #[xml(child = "sound_occlusion")]
    pub sound_occlusion: Option<SoundOccu>,
    #[xml(child = "cargo")]
    pub cargo: Option<Cargo>,
    #[xml(child = "sounds")]
    pub sounds: Option<Sounds>,
    #[xml(child = "capture")]
    pub capture: Option<Capture>,
    #[xml(child = "effects")]
    pub effects: Option<Effects>,
    #[xml(child = "docksize")]
    pub docksize: Option<Docksize>,
    #[xml(child = "enginearticulation")]
    pub enginearticulation: Option<Enginearticulation>,
    #[xml(child = "rotationspeed")]
    pub rotationspeed: Option<Rotationspeed>,
    #[xml(child = "rotationacceleration")]
    pub rotationacceleration: Option<Rotationacceleration>,
    #[xml(child = "storage")]
    pub storageamt: Option<StorageAmnt>,
    #[xml(child = "gatherrate")]
    pub gatherrate: Option<GatherRate>,
    #[xml(child = "wall")]
    pub wall: Option<Wall>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "wall")]
pub struct Wall {
    #[xml(attr = "opaque")]
    pub opaque: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "gatherrate")]
pub struct GatherRate {
    #[xml(attr = "gas")]
    pub gas: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "storage")]
pub struct StorageAmnt {
    #[xml(attr = "countermeasure")]
    pub countermeasure: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "rotationspeed")]
pub struct Rotationspeed {
    #[xml(attr = "max")]
    pub max: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "rotationacceleration")]
pub struct Rotationacceleration {
    #[xml(attr = "max")]
    pub max: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "enginearticulation")]
pub struct Enginearticulation {
    #[xml(attr = "y")]
    pub y: Option<String>,
    #[xml(attr = "z")]
    pub z: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "docksize")]
pub struct Docksize {
    #[xml(attr = "tag")]
    pub tag: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "effects")]
pub struct Effects {
    #[xml(child = "explosion")]
    pub effects: Option<Explosion>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "explosion")]
pub struct Explosion {
    #[xml(attr = "ref")]
    pub refexplo: Option<String>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "capture")]
pub struct Capture {
    #[xml(attr = "allow")]
    pub allow: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "sounds")]
pub struct Sounds {
    #[xml(child = "shipdetail")]
    pub shipdetail: Option<Shipdetail>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "shipdetail")]
pub struct Shipdetail {
    #[xml(attr = "ref")]
    pub shipdetailref: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "cargo")]
pub struct Cargo {
    #[xml(attr = "max")]
    pub max: Option<String>,
        #[xml(attr = "tags")]
    pub tags: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "sound_occlusion")]
pub struct SoundOccu {
    #[xml(attr = "inside")]
    pub inside: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "loadouts")]
pub struct Loadouts {
    #[xml(child = "loadout")]
    pub loadouts: Option<Loadout>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "loadout")]
pub struct Loadout {
    #[xml(child = "macros")]
    pub loadout: Option<LoadoutMacros>,
    #[xml(child = "groups")]
    pub groups: Option<LoadoutGroups>,
    #[xml(child = "ammunition")]
    pub ammunition: Option<LoadoutAmmo>,
    #[xml(child = "virtualmacros")]
    pub virtualmacros: Option<VirtualMacros>,
    #[xml(attr = "id")]
    pub id: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "virtualmacros")]
pub struct VirtualMacros {
    #[xml(child = "virtualmacro")]
    pub thruster: Option<VirtualMacro>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "virtualmacro")]
pub struct VirtualMacro {
    #[xml(attr = "macro")]
    pub virtualmacromacro: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "ammunition")]
pub struct LoadoutAmmo {
    #[xml(child = "ammunition")]
    pub ammunition: Option<Ammunition>,
    #[xml(child = "turrets")]
    pub turrets: Vec<LoadoutTurrets>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "ammunition")]
pub struct Ammunition {
    #[xml(attr = "macro")]
    pub ammomacro: Option<String>,
    #[xml(attr = "min")]
    pub min: Option<String>,
    #[xml(attr = "exact")]
    pub exact: Option<String>,
    #[xml(attr = "max")]
    pub max: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "groups")]
pub struct LoadoutGroups {
    #[xml(child = "shields")]
    pub shields: Vec<LoadoutShields>,
    #[xml(child = "turrets")]
    pub turrets: Vec<LoadoutTurrets>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "shields")]
pub struct LoadoutShields {
    #[xml(attr = "macro")]
    pub shmacro: Option<String>,
    #[xml(attr = "shield")]
    pub path: Option<String>,
    #[xml(attr = "min")]
    pub min: Option<String>,
    #[xml(attr = "exact")]
    pub exact: Option<String>,
    #[xml(attr = "max")]
    pub max: Option<String>,
    #[xml(attr = "group")]
    pub group: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "turrets")]
pub struct LoadoutTurrets {
    #[xml(attr = "macro")]
    pub turmacro: Option<String>,
    #[xml(attr = "path")]
    pub path: Option<String>,
    #[xml(attr = "min")]
    pub min: Option<String>,
    #[xml(attr = "exact")]
    pub exact: Option<String>,
    #[xml(attr = "max")]
    pub max: Option<String>,
    #[xml(attr = "group")]
    pub group: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "macros")]
pub struct LoadoutMacros {
    #[xml(child = "weapon")]
    pub weapon: Vec<LoadoutWeapon>,
    #[xml(child = "turret")]
    pub turret: Vec<LoadoutTurret>,
    #[xml(child = "engine")]
    pub engine: Vec<LoadoutEngine>,
    #[xml(child = "shield")]
    pub shield: Vec<LoadoutShield>,
    #[xml(attr = "id")]
    pub id: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "turret")]
pub struct LoadoutTurret {
    #[xml(attr = "macro")]
    pub turreteng: Option<String>,
    #[xml(attr = "path")]
    pub path: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "weapon")]
pub struct LoadoutWeapon {
    #[xml(attr = "macro")]
    pub weaponeng: Option<String>,
    #[xml(attr = "path")]
    pub path: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "engine")]
pub struct LoadoutEngine {
    #[xml(attr = "macro")]
    pub macroeng: Option<String>,
    #[xml(attr = "path")]
    pub path: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "shield")]
pub struct LoadoutShield {
    #[xml(attr = "macro")]
    pub macroshi: Option<String>,
    #[xml(attr = "path")]
    pub path: Option<String>,
    #[xml(attr = "optional")]
    pub optional: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "explosiondamage")]
pub struct ExplosionDam {
    #[xml(attr = "value")]
    pub explosion: Option<String>,
    #[xml(attr = "time")]
    pub time: Option<String>,
    #[xml(attr = "hull")]
    pub hull: Option<String>,
    
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "storage")]
pub struct Storage {
    #[xml(attr = "unit")]
    pub unit: Option<String>,
    #[xml(attr = "missile")]
    pub missile: Option<String>,
    #[xml(attr = "countermeasure")]
    pub countermeasure: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "hull")]
pub struct Hull {
    #[xml(attr = "max")]
    pub max: Option<String>,
    #[xml(attr = "integrated")]
    pub integrated: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "secrecy")]
pub struct Secrecy {
    #[xml(attr = "level")]
    pub level: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "purpose")]
pub struct Purpose {
    #[xml(attr = "primary")]
    pub primary: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "people")]
pub struct People {
    #[xml(attr = "capacity")]
    pub capacity: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "physics")]
pub struct Physics {
    #[xml(child = "inertia")]
    pub inertia: Option<Inertia>,
    #[xml(child = "drag")]
    pub drag: Option<Drag>,
    #[xml(attr = "mass")]
    pub mass: Option<String>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "inertia")]
pub struct Inertia {
    #[xml(attr = "pitch")]
    pub pitch: Option<String>,
    #[xml(attr = "yaw")]
    pub yaw: Option<String>,
    #[xml(attr = "roll")]
    pub roll: Option<String>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "drag")]
pub struct Drag {
    #[xml(attr = "forward")]
    pub forward: Option<String>,
    #[xml(attr = "reverse")]
    pub reverse: Option<String>,
    #[xml(attr = "horizontal")]
    pub horizontal: Option<String>,
    #[xml(attr = "vertical")]
    pub vertical: Option<String>,
    #[xml(attr = "pitch")]
    pub pitch: Option<String>,
    #[xml(attr = "yaw")]
    pub yaw: Option<String>,
    #[xml(attr = "roll")]
    pub roll: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "thruster")]
pub struct Thruster {
    #[xml(attr = "tags")]
    pub tags: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "ship")]
pub struct Ship {
    #[xml(attr = "type")]
    pub shiptype: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "software")]
pub struct Software {
    #[xml(child = "software")]
    pub software: Vec<SoftwareWares>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "software")]
pub struct SoftwareWares {
    #[xml(attr = "software")]
    pub software: Option<String>,
    #[xml(attr = "compatible")]
    pub compatible: Option<String>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(attr = "ware")]
    pub ware: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "identification")]
pub struct Identification {
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "basename")]
    pub basename: Option<String>,
    #[xml(attr = "description")]
    pub description: Option<String>,
    #[xml(attr = "variation")]
    pub variation: Option<String>,
    #[xml(attr = "shortvariation")]
    pub shortvariation: Option<String>,
    #[xml(attr = "icon")]
    pub icon: Option<String>,
    #[xml(attr = "unique")]
    pub unique: Option<String>,
    #[xml(attr = "makerrace")]
    pub makerrace: Option<String>,
    #[xml(attr = "unit")]
    pub unit: Option<String>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "connections")]
pub struct Connections {
    #[xml(child = "connection")]
    pub connection: Vec<Connection>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "connection")]
pub struct Connection {
    #[xml(attr = "ref")]
    pub refer: Option<String>,
    #[xml(child = "macro")]
    pub macroref: Option<MacroRef>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "macro")]
pub struct MacroRef {
    #[xml(attr = "ref")]
    pub refer: Option<String>,
    #[xml(attr = "connection")]
    pub connection: Option<String>,
}
