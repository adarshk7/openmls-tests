#[test]
fn test_two_member_group() {
    // Reset the server state.
    backend::Backend::default().reset_server();

    const GROUP_NAME: &str = "test_group";

    // Client two clients.
    let mut client_1 = user::User::new("client_1".to_string());
    let mut client_2 = user::User::new("client_2".to_string());

    // Update the clients to update them to latest server state.
    client_1.update(None).unwrap();
    client_2.update(None).unwrap();

    // Client 1 creates a group.
    client_1.create_group(GROUP_NAME.to_string());

    // Client 1 adds Client 2 to the group.
    client_1.invite("client_2".to_string(), GROUP_NAME.to_string()).unwrap();

    // Update the clients to update them to latest server state.
    client_1.update(None).unwrap();
    client_2.update(None).unwrap();

    const MESSAGE: &str = "Hello, world!";

    // Client 2 sends a message.
    client_2.send_msg(MESSAGE, GROUP_NAME.to_string()).unwrap();

    // Update the clients to update them to latest server state.
    client_1.update(None).unwrap();
    client_2.update(None).unwrap();

    assert_eq!(
        client_1.read_msgs(GROUP_NAME.to_string()).unwrap(),
        Some(vec![MESSAGE.into()])
    );
}

