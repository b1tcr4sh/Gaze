use zbus::dbus_proxy;
use zbus::Connection;
use zvariant::ObjectPath;

use self::profile_model::*;
use self::profile_mess_model::*;

mod profile_model;
mod profile_mess_model;

#[dbus_proxy(
    interface = "org.mercurius.profile",
    default_service = "org.mercurius.profile",
    default_path = "/org/mercurius/profile/"
)]
trait Profile {
    fn GetProfileInfo(&self) -> zbus::Result<ProfileInfo>;
    fn AddMod(&self, project_id: String, service: Repo, ignore_dependencies: bool) -> zbus::Result<Mod>;
    fn RemoveMod(&self, version_id: String, force: bool) -> zbus::Result<bool>;
    fn Sync(&self) -> zbus::Result<bool>;
    fn ListMods(&self) -> zbus::Result<Vec<Mod>>;
    fn Verify(&self) -> zbus::Result<ValidityReport>;
}

pub async fn Connect() -> Connection {
    let connection = Connection::session().await;

    match connection {
        Ok(connection ) => {
            return connection;
        },
        Err(why) => {
            panic!("Failed to connect to session bus!: {:?}", why);
        }
    }
}
pub async fn GetProfiles(&connection: Connection ) -> Vec<ProfileProxy> {
    let Ok(messenger) = ProfileMessengerProxy::new(connection).await;

    profiles = messenger.ListProfiles();
}