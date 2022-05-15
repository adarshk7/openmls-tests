#[test]
fn statistics_test() {
    use std::time::Instant;

    backend::Backend::default().reset_server();

    const MESSAGE_1: &str = "Hello, world!";
    const N_MESSAGES: i32 = 10;
    const N_MEMBERS: i32 = 25;
    let mut main_client = user::User::new("main_client".to_string());

    // Create clients
    let mut clients: Vec<user::User> = Vec::new();
    for i in 0..N_MEMBERS {
        let client = user::User::new(format!("client_{}", i));
        clients.push(client);
    }
    
    // Update clients
    main_client.update(None).unwrap();
    for client in &mut clients {
        client.update(None).unwrap();
    }

    // Create group
    main_client.create_group("test_group".to_string());
    main_client.update(None).unwrap();

    let invite_start = Instant::now();
    // Invite everyone
    for i in 0..N_MEMBERS {
        main_client.invite(format!("client_{}", i), "test_group".to_string()).unwrap();
    }

    // Update clients
    main_client.update(None).unwrap();
    for client in &mut clients {
        client.update(None).unwrap();
    }
    println!("Inviting {} members took: {:.2?}", N_MEMBERS, invite_start.elapsed());

    let send_message_start = Instant::now();
    // Send messages
    for _i in 0..N_MESSAGES {
        main_client.send_msg(MESSAGE_1, "test_group".to_string()).unwrap();
    }
    println!("Sending {} messages took: {:.2?}", N_MESSAGES, send_message_start.elapsed());

    // Update clients
    let read_message_start = Instant::now();
    main_client.update(None).unwrap();
    for client in &mut clients {
        client.update(None).unwrap();
    }

    for client in &mut clients {
        client.read_msgs("test_group".to_string()).unwrap();
    }
    println!("Reading {} messages took: {:.2?}", N_MESSAGES, read_message_start.elapsed());
}

