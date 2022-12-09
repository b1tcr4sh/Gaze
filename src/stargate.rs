use zbus::dbus_proxy;
use zbus::Connection;

use self::profile_model::*;
use self::profile_mess_model::*;

pub mod profile_model;
mod profile_mess_model;

pub static Loaders: [&str; 7] = ["unknown", "Forge", "Fabric", "Quilt", "Lite Loader", "Rift", "ModLoader"];

#[dbus_proxy(
    interface = "org.mercurius.profile",
    default_service = "org.mercurius.profile"
)]
trait Profile {
    fn GetProfileInfo(&self) -> zbus::Result<ProfileInfo>;
    fn AddMod(&self, project_id: String, service: Repo, ignore_dependencies: bool) -> zbus::Result<Mod>;
    fn RemoveMod(&self, version_id: String, force: bool) -> zbus::Result<bool>;
    fn Sync(&self) -> zbus::Result<bool>;
    fn ListMods(&self) -> zbus::Result<Vec<Mod>>;
    fn Verify(&self) -> zbus::Result<ValidityReport>;
}

pub async fn connect() -> Connection {
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
pub async fn get_profile_messenger<'a>(connection: &'a Connection) -> ProfileMessengerProxy<'a> {
    let result = ProfileMessengerProxy::new(connection).await;

    match result {
        Ok(messenger) => {
            return messenger;
        },
        Err(why) => {
            panic!("{:?}", why);
        }
    }
}

pub async fn get_profiles<'a>(connection: &'a Connection, messenger: ProfileMessengerProxy<'a>) -> Vec<ProfileProxy<'a>> {
    let profiles_names = messenger.ListProfiles().await;
    let mut profiles: Vec<ProfileProxy<'a>> = Vec::new();

    match profiles_names {
        Ok(names) => {
            for profile in names {
                let builder = ProfileProxy::builder(connection).path(["/org/mercurius/profile/", &profile].join(""));       
                
                match builder {
                    Ok(builder) => {
                        let profile = builder.build().await;

                        match profile {
                            Ok(proxy) => {
                                profiles.insert(0, proxy);
                            },
                            Err(why) => {
                                panic!("{:?}", why);
                            }
                        }
                    },
                    Err(why) => {
                        panic!("{:?}", why);
                    }
                }
            }
        },
        Err(why) => {
            panic!("{:?}", why);
        }
    }

    profiles
}