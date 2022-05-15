#[test]
fn test_add_member() {
    // Reset the server state.
    backend::Backend::default().reset_server();

    const GROUP_NAME: &str = "test_group";

    // Client two clients.
    let mut client_1 = user::User::new("client_1".to_string());
    let mut client_2 = user::User::new("client_2".to_string());
    let mut client_3 = user::User::new("client_3".to_string());

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

    // Client 1 adds Client 3 to the group.
    client_1.invite("client_3".to_string(), GROUP_NAME.to_string()).unwrap();

    // Update the clients to update them to latest server state.
    client_1.update(None).unwrap();
    client_2.update(None).unwrap();
    client_3.update(None).unwrap();

    const NEW_MESSAGE: &str = "Hello, world! 2";

    // Client 2 sends a message again.
    client_2.send_msg(NEW_MESSAGE, GROUP_NAME.to_string()).unwrap();

    // Update the clients to update them to latest server state.
    client_1.update(None).unwrap();
    client_2.update(None).unwrap();
    client_3.update(None).unwrap();

    // Client 1 sees both messages.
    assert_eq!(
        client_1.read_msgs(GROUP_NAME.to_string()).unwrap(),
        Some(vec![MESSAGE.into(), NEW_MESSAGE.into()])
    );

    // Client 2 only sees the new message.
    assert_eq!(
        client_3.read_msgs(GROUP_NAME.to_string()).unwrap(),
        Some(vec![NEW_MESSAGE.into()])
    );
}
