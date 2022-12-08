use zbus::dbus_proxy;
use zvariant::ObjectPath;

use crate::stargate::profile_model::{Loader};

#[dbus_proxy(
    interface = "org.mercurius.ProfileMessenger",
    default_service = "org.mercurius.ProfileMessenger",
    default_path = "/org/mercurius/ProfileMessenger"
)]
trait ProfileMessenger {
    fn ListProfiles(&self) -> zbus::Result<Vec<ObjectPath>>;
    fn CreateProfile(&self, name: String, minecraft_version: String, loader: Loader, server_side: bool) -> zbus::Result<ObjectPath>;
    // fn DeleteProfile(&self, name: String) -> Result<()>;
}