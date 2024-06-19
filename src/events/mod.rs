use crate::shared::{Address, WorkspaceId, WorkspaceType};
use serde::Deserialize;

mod parse;

/// This enum holds every event type
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub enum Event {
    WorkspaceChanged(WorkspaceType),
    WorkspaceDeleted(WorkspaceDestroyedEventData),
    WorkspaceAdded(WorkspaceType),
    WorkspaceMoved(MonitorEventData),
    WorkspaceRename(WorkspaceRenameEventData),
    ActiveWindowChangedV1(Option<(String, String)>),
    ActiveWindowChangedV2(Option<Address>),
    ActiveWindowChangedMerged(Option<WindowEventData>),
    ActiveMonitorChanged(MonitorEventData),
    FullscreenStateChanged(bool),
    MonitorAdded(String),
    MonitorRemoved(String),
    WindowOpened(WindowOpenEvent),
    WindowClosed(Address),
    WindowMoved(WindowMoveEvent),
    LayoutChanged(LayoutEvent),
    SubMapChanged(String),
    LayerOpened(String),
    LayerClosed(String),
    FloatStateChanged(WindowFloatEventData),
    UrgentStateChanged(Address),
    Minimize(MinimizeEventData),
    WindowTitleChanged(Address),
    Screencast(ScreencastEventData),
}

/// This tuple struct holds window event data
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WindowEventData {
    /// The window class
    pub window_class: String,
    /// The window title
    pub window_title: String,
    /// The window address
    pub window_address: Address,
}

/// This tuple struct holds monitor event data
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MonitorEventData {
    /// The monitor name
    pub monitor_name: String,
    /// The workspace
    pub workspace: WorkspaceType,
}

/// This tuple struct holds monitor event data
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WindowFloatEventData {
    /// The window address
    pub window_address: Address,
    /// The float state
    pub is_floating: bool,
}

/// The data for the event executed when changing keyboard layouts
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct LayoutEvent {
    /// Keyboard name
    pub keyboard_name: String,
    /// Layout name
    pub layout_name: String,
}

/// Event data for destroyworkspacev2 event
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WorkspaceDestroyedEventData {
    /// Workspace Id
    pub workspace_id: WorkspaceId,
    /// Workspace name
    pub workspace_name: String,
}

/// Event data for renameworkspace event
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WorkspaceRenameEventData {
    /// Workspace id
    pub workspace_id: WorkspaceId,
    /// Workspace name content
    pub workspace_name: String,
}

/// Event data for a minimize event
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct MinimizeEventData {
    /// Window address
    pub window_address: Address,
    /// whether it's minimized or not
    pub is_minimized: bool,
}

/// Event data for screencast event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct ScreencastEventData {
    /// State/Is it turning on?
    pub is_turning_on: bool,
    /// Owner type, is it a monitor?
    pub is_monitor: bool,
}

/// The data for the event executed when moving a window to a new workspace
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct WindowMoveEvent {
    /// Window address
    pub window_address: Address,
    /// The workspace name
    pub workspace_name: String,
}

/// The data for the event executed when opening a new window
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct WindowOpenEvent {
    /// Window address
    pub window_address: Address,
    /// The workspace name
    pub workspace_name: String,
    /// Window class
    pub window_class: String,
    /// Window title
    pub window_title: String,
}
