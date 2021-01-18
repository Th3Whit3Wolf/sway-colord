use anyhow::Result;
use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
    fn set_environment(&self, assignments: Vec<&str>) -> zbus::Result<()>;
}

pub fn set_environment(env: Vec<&str>) -> Result<()> {
    let connection = zbus::Connection::new_session()?;
    let proxy = SystemdManagerProxy::new(&connection)?;
    proxy.set_environment(env)?;
    Ok(())
}