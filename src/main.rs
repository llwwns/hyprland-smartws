use anyhow::Result;
use hyprland::data::Monitors;
use hyprland::dispatch::*;
use hyprland::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }
    let id = args[1].parse::<i32>()?;
    let monitors = Monitors::get()?;
    let mut other = i16::MAX;
    let mut current = i16::MAX;
    for monitor in monitors {
        if monitor.focused {
            if monitor.active_workspace.id == id {
                return Ok(());
            }
            current = monitor.id;
        }
        if monitor.active_workspace.id == id {
            other = monitor.id;
        }
    }
    if current == i16::MAX {
        return Ok(());
    }
    if other == i16::MAX {
        Dispatch::call(DispatchType::MoveWorkspaceToMonitor(
            WorkspaceIdentifier::Id(id),
            MonitorIdentifier::Id(current as u8),
        ))?;
        Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
            id,
        )))?;
    } else {
        Dispatch::call(DispatchType::SwapActiveWorkspaces(
            MonitorIdentifier::Id(current as u8),
            MonitorIdentifier::Id(other as u8),
        ))?;
    }
    Ok(())
}
