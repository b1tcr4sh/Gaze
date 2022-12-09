use zvariant::Type;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub struct ProfileInfo {
    pub name: String,
    pub minecraft_version: String,
    pub is_server_side: bool,
    pub loader: i32, // Tmds.Dbus serializes enums as i32, zvariant expects u32 for enums
    pub file_path: String
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub struct ValidityReport {
    incompatible: Vec<Mod>,
    missing_dependencies: Vec<String>,
    synced: bool,
    repaired: bool
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub struct Mod {
    title: String,
    file_name: String,
    download_url: String,
    project_id: String,
    version_id: String,
    mc_version: String,
    version: String,
    loaders: Vec<Loader>,
    dependencies: Vec<String>,
    client_dependency: ClientDependency
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub enum Repo {
    Modrinth, Curseforge, Custom
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub enum Loader {
    Unknown = 0, Forge = 1, Fabric = 2, Quilt = 3, Liteloader = 4, Rift = 5, Modloader = 6
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub enum ClientDependency {
    ClientSide, ServerSide, ClientServerDependent, Unknown
}