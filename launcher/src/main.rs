#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Result, bail};
use std::ffi::OsStr;
use std::path::PathBuf;

use goxlr_ipc::client::Client;
use goxlr_ipc::clients::ipc::ipc_client::IPCClient;
use goxlr_ipc::clients::ipc::ipc_socket::Socket;
use goxlr_ipc::{DaemonCommand, DaemonRequest, DaemonResponse};
use interprocess::local_socket::tokio::prelude::LocalSocketStream;
use interprocess::local_socket::traits::tokio::Stream;
use interprocess::local_socket::{GenericNamespaced, ToNsName};
use which::which;

static NAMED_PIPE: &str = "@goxlr.socket";
static DAEMON_NAME: &str = "goxlr-daemon";

#[tokio::main]
async fn main() -> Result<()> {
    // First thing to do, is check to see if the Daemon is running..
    if !is_daemon_running() {
        launch_daemon()?;
    }

    open_ui().await?;
    Ok(())
}

async fn get_connection() -> Result<LocalSocketStream> {
    let path = NAMED_PIPE.to_ns_name::<GenericNamespaced>();

    let path = match path {
        Ok(path) => path,
        Err(e) => {
            bail!("Unable to Process Path {}", e);
        }
    };

    LocalSocketStream::connect(path)
        .await
        .map_err(anyhow::Error::msg)
}

fn is_daemon_running() -> bool {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};

    let process_refresh_kind = ProcessRefreshKind::everything().without_tasks();
    let refresh_kind = RefreshKind::nothing().with_processes(process_refresh_kind);
    let system = System::new_with_specifics(refresh_kind);

    let binding = get_daemon_binary_name();
    let processes = system.processes_by_exact_name(OsStr::new(&binding));

    processes.count() > 0
}

fn launch_daemon() -> Result<()> {
    use std::process::{Command, Stdio, exit};

    if let Some(path) = locate_daemon_binary() {
        let mut command = Command::new(&path);
        command.arg("--start-ui");
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());

        if let Some(parent) = path.parent() {
            command.current_dir(parent);
        }

        command.spawn().expect("Unable to Launch Child Process");
        exit(0);
    }

    bail!("Unable to Locate GoXLR Daemon Binary");
}

async fn open_ui() -> Result<()> {
    let mut usable_connection = None;

    if let Ok(connection) = get_connection().await {
        usable_connection.replace(connection);
    }

    if let Some(connection) = usable_connection {
        let socket: Socket<DaemonResponse, DaemonRequest> = Socket::new(connection);
        let mut client = IPCClient::new(socket);
        client
            .send(DaemonRequest::Daemon(DaemonCommand::Activate))
            .await?;
        return Ok(());
    }
    bail!("Unable to make a connection with the Daemon");
}

fn locate_daemon_binary() -> Option<PathBuf> {
    let mut binary_path = None;
    let bin_name = get_daemon_binary_name();

    let cwd = std::env::current_dir().unwrap().join(bin_name.clone());
    if cwd.exists() {
        binary_path.replace(cwd);
    }

    if binary_path.is_none()
        && let Some(parent) = std::env::current_exe().unwrap().parent()
    {
        let bin = parent.join(bin_name.clone());
        if bin.exists() {
            binary_path.replace(bin);
        }
    }

    if binary_path.is_none() {
        if let Ok(path) = which(bin_name) {
            binary_path.replace(path);
        }
    }

    binary_path
}

fn get_daemon_binary_name() -> String {
    format!("{DAEMON_NAME}.exe")
}
