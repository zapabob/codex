use super::*;

#[test]
fn test_parse_command() {
    let communicator = LineCommunicator::default();

    // Test code execution command
    let command = communicator.parse_command("/code python print('hello')");
    assert!(matches!(
        command,
        Some(DevelopmentCommand::ExecuteCode { .. })
    ));

    // Test file creation command
    let file_command = communicator.parse_command("/file test.txt Hello World");
    assert!(matches!(
        file_command,
        Some(DevelopmentCommand::CreateFile { .. })
    ));

    // Test invalid command
    let invalid = communicator.parse_command("invalid command");
    assert!(invalid.is_none());
}

#[test]
fn test_session_management() {
    let communicator = LineCommunicator::default();

    // Start session
    communicator.start_development_session("user123", "Test User");

    // Check session exists
    let sessions = communicator.get_active_sessions();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].user_id, "user123");

    // End session
    communicator.end_development_session("user123");

    // Check session is removed
    let sessions_after = communicator.get_active_sessions();
    assert_eq!(sessions_after.len(), 0);
}
