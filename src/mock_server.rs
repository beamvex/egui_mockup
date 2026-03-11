use crate::model::{Channel, Message, Server};

pub fn mock_servers() -> Vec<Server> {
    vec![
        Server {
            name: "Rust Lounge".to_owned(),
            channels: vec![
                Channel {
                    name: "general".to_owned(),
                    messages: vec![
                        Message {
                            author: "alice".to_owned(),
                            content: "Welcome! This is a mock Discord-like client built with egui."
                                .to_owned(),
                            timestamp: "10:12".to_owned(),
                        },
                        Message {
                            author: "bob".to_owned(),
                            content: "Try selecting channels and sending a message.".to_owned(),
                            timestamp: "10:13".to_owned(),
                        },
                    ],
                },
                Channel {
                    name: "egui".to_owned(),
                    messages: vec![
                        Message {
                            author: "carol".to_owned(),
                            content: "egui makes it easy to prototype UIs.".to_owned(),
                            timestamp: "09:58".to_owned(),
                        },
                        Message {
                            author: "dave".to_owned(),
                            content: "Next step could be avatars, markdown, and message grouping."
                                .to_owned(),
                            timestamp: "10:01".to_owned(),
                        },
                    ],
                },
                Channel {
                    name: "help".to_owned(),
                    messages: vec![Message {
                        author: "moderator".to_owned(),
                        content: "Type in the composer and hit Enter to send.".to_owned(),
                        timestamp: "08:00".to_owned(),
                    }],
                },
            ],
        },
        Server {
            name: "Game Dev".to_owned(),
            channels: vec![
                Channel {
                    name: "announcements".to_owned(),
                    messages: vec![Message {
                        author: "admin".to_owned(),
                        content: "Mock announcements go here.".to_owned(),
                        timestamp: "yesterday".to_owned(),
                    }],
                },
                Channel {
                    name: "screenshots".to_owned(),
                    messages: vec![Message {
                        author: "eve".to_owned(),
                        content: "(pretend there are images)".to_owned(),
                        timestamp: "11:02".to_owned(),
                    }],
                },
            ],
        },
        Server {
            name: "Friends".to_owned(),
            channels: vec![Channel {
                name: "chat".to_owned(),
                messages: vec![
                    Message {
                        author: "sam".to_owned(),
                        content: "Hey!".to_owned(),
                        timestamp: "Sun".to_owned(),
                    },
                    Message {
                        author: "you".to_owned(),
                        content: "Yo.".to_owned(),
                        timestamp: "Sun".to_owned(),
                    },
                ],
            }],
        },
    ]
}
