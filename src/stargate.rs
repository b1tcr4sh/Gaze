use zbus::dbus_proxy;

use self::data_models::*;

mod data_models;

#[dbus_proxy(
    interface = "org.mercurius.profile",
    default_service = "org.mercurius.profile",
    default_path = "/org/mercurius/profile/uwu"
)]
trait Profile {
    fn GetProfileInfo(&self) -> zbus::Result<ProfileInfo>;
    fn AddMod(&self, project_id: String, service: Repo, ignore_dependencies: bool) -> zbus::Result<Mod>;
    fn RemoveMod(&self, version_id: String, force: bool) -> zbus::Result<bool>;
    fn Sync(&self) -> zbus::Result<bool>;
    fn ListMods(&self) -> zbus::Result<Vec<Mod>>;
    fn Verify(&self) -> zbus::Result<ValidityReport>;
}

