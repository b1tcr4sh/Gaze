use zbus::dbus_proxy;

use crate::stargate::profile_model::{Loader};

#[dbus_proxy(
interface = "org.mercurius.ProfileMessenger",
default_service = "org.mercurius.ProfileMessenger",
default_path = "/org/mercurius/ProfileMessenger"
)]
trait ProfileMessenger {
    fn ListProfiles<'a>(&self) -> zbus::Result<Vec<String>>; // Can't serielazise ObjectPath to String ahhhhhhh
    fn CreateProfile<'a>(&self, name: String, minecraft_version: String, loader: Loader, server_side: bool) -> zbus::Result<String>;
    // fn DeleteProfile(&self, name: String) -> Result<()>;
}