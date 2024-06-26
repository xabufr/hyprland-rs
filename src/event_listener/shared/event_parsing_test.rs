use crate::event_listener::{
    event_parser, Event, MonitorEventData, WorkspaceRenameEventData, WorkspaceType, WorkspaceV2Data,
};

#[test]
fn test_parsing_unknown_event() {
    let events = r#"unknowevent>>2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(parsed, vec![])
}

#[test]
fn test_parsing_bad_id() {
    let events = r#"createworkspacev2>>NOT_A_NUMBER,name"#;
    let error = event_parser(events.into()).err();
    assert!(error.is_some());
    assert_eq!(
        format!("{}", error.unwrap()),
        "Cannot parse workspace id: invalid integer error: invalid digit found in string"
    )
}

#[test]
fn test_parsing_createworkspace() {
    let events = r#"createworkspace>>2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceAdded(WorkspaceType::Regular("2".into()))]
    )
}

#[test]
fn test_parsing_moveworkspace() {
    let events = r#"moveworkspace>>2,monitor-1"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceMoved(MonitorEventData {
            monitor_name: "monitor-1".into(),
            workspace: WorkspaceType::Regular("2".into()),
        })]
    )
}

#[test]
fn test_parsing_createworkspacev2() {
    let events = r#"createworkspacev2>>2,name-2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceAddedV2(WorkspaceV2Data {
            workspace_id: 2,
            workspace_name: WorkspaceType::Regular("name-2".into()),
        })]
    )
}

#[test]
fn test_parsing_destroyworkspacev2() {
    let events = r#"destroyworkspacev2>>2,name-2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceDeleted(WorkspaceV2Data {
            workspace_id: 2,
            workspace_name: WorkspaceType::Regular("name-2".into()),
        })]
    )
}

#[test]
fn test_parsing_createworkspacev2_special() {
    let events = r#"createworkspacev2>>-98,special:name-2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceAddedV2(WorkspaceV2Data {
            workspace_id: -98,
            workspace_name: WorkspaceType::Special(Some("name-2".into())),
        })]
    )
}

#[test]
fn test_parsing_workspacerename() {
    let events = r#"renameworkspace>>3,new name"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceRename(WorkspaceRenameEventData {
            workspace_id: 3,
            workspace_name: "new name".into(),
        })]
    )
}

#[test]
fn test_parsing_multiple_events() {
    let events = r#"createworkspace>>2
createworkspacev2>>2,named 2
"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![
            Event::WorkspaceAdded(WorkspaceType::Regular("2".into())),
            Event::WorkspaceAddedV2(WorkspaceV2Data {
                workspace_id: 2,
                workspace_name: WorkspaceType::Regular("named 2".into()),
            }),
        ]
    )
}
