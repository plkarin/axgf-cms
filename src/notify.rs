//! Telling systemd what this process is doing.
//!
//! # Why this is not a dependency
//!
//! The protocol is a datagram of `KEY=value` lines to the socket named by
//! `$NOTIFY_SOCKET`. That is the whole of it, and it is stable — `sd_notify(3)`
//! documents exactly this wire format — so a crate, a C library and a linkage
//! to libsystemd would all be ceremony around eleven lines of `UnixDatagram`.
//!
//! # What it buys
//!
//! Two things an operator sees without reading logs.
//!
//! `READY=1` makes the unit `Type=notify`, which means "started" stops meaning
//! "the process was spawned" and starts meaning "the bundle is loaded and the
//! socket is listening". On the operator's 435 MB archive that is several
//! seconds of difference, and it is the difference between `systemctl start`
//! returning into a working site and returning into a 502.
//!
//! `STATUS=` is the line `systemctl status` prints under the unit: what bundle
//! is loaded, how many people are in it, and how long ago the last backup was.
//! Those are the three questions an operator opens `status` to answer, and
//! without this they are answerable only by reading the journal.

use std::io;
use std::os::unix::net::UnixDatagram;

/// Send one notification. `false` when there is nothing to notify — not under
/// systemd, or the socket has gone.
///
/// Never an error the caller has to handle: nothing this reports is worth
/// failing a startup over, and a service that refuses to run because it could
/// not describe itself would be a poor trade.
pub fn notify(message: &str) -> bool {
    match send(message) {
        Ok(()) => true,
        Err(e) => {
            // Debug, not warn: "not under systemd" is the normal case for a
            // developer running this from a terminal.
            tracing::debug!(error = %e, "could not notify systemd");
            false
        }
    }
}

/// `READY=1`, plus the first status line.
pub fn ready(status: &str) -> bool {
    notify(&format!("READY=1\nSTATUS={status}"))
}

/// Replace the line `systemctl status` shows.
pub fn status(status: &str) -> bool {
    notify(&format!("STATUS={status}"))
}

/// `STOPPING=1`, so a slow shutdown reads as "stopping" rather than as a unit
/// that has stopped answering.
pub fn stopping(status: &str) -> bool {
    notify(&format!("STOPPING=1\nSTATUS={status}"))
}

fn send(message: &str) -> io::Result<()> {
    let path = std::env::var_os("NOTIFY_SOCKET")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "NOTIFY_SOCKET is not set"))?;
    let path = path.to_string_lossy().into_owned();
    let sock = UnixDatagram::unbound()?;
    // A leading '@' names an abstract socket, which is written with a leading
    // NUL on the wire. systemd uses a filesystem path for system services and
    // an abstract name for some user ones; both have to work.
    if let Some(name) = path.strip_prefix('@') {
        use std::os::linux::net::SocketAddrExt as _;
        let addr = std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes())?;
        sock.send_to_addr(message.as_bytes(), &addr)?;
    } else {
        sock.send_to(message.as_bytes(), &path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_happens_without_the_socket() {
        // The developer's terminal. Not an error, not a panic, just false.
        let had = std::env::var_os("NOTIFY_SOCKET");
        std::env::remove_var("NOTIFY_SOCKET");
        assert!(!ready("whatever"));
        if let Some(v) = had {
            std::env::set_var("NOTIFY_SOCKET", v);
        }
    }

    #[test]
    fn a_datagram_arrives_with_the_lines_it_was_given() {
        let dir = crate::scratch::Dir::new("notify");
        let path = dir.join("notify.sock");
        let server = UnixDatagram::bind(&path).expect("bind");
        std::env::set_var("NOTIFY_SOCKET", &path);

        assert!(ready("loaded, 866 people"));

        let mut buf = [0u8; 256];
        let n = server.recv(&mut buf).expect("recv");
        let got = String::from_utf8_lossy(&buf[..n]).into_owned();
        assert_eq!(got, "READY=1\nSTATUS=loaded, 866 people");
        std::env::remove_var("NOTIFY_SOCKET");
    }
}
