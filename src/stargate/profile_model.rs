use zvariant::Type;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub struct ProfileInfo {
    name: String,
    minecraft_version: String,
    is_server_side: String,
    loader: i32,
    file_path: String
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
    Unknown, Forge, Fabric, Quilt, Liteloader, Rift, Modloader
}
#[derive(Serialize, Deserialize, Debug, Type, PartialEq)]
pub enum ClientDependency {
    ClientSide, ServerSide, ClientServerDependent, Unknown
}