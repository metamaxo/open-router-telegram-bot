use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub chat: Chat,
    pub date: i64,
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    pub offset: i64,
    pub length: i64,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: Option<bool>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Chat {
    Private {
        id: i64,
        first_name: String,
        last_name: Option<String>,
        username: Option<String>,
    },
    Group {
        id: i64,
        title: String,
        all_members_are_administrators: Option<bool>,
    },
    Supergroup {
        id: i64,
        title: String,
        all_members_are_administrators: Option<bool>,
    },
    Channel {
        id: i64,
        title: String,
    },
}

impl Chat {
    pub fn get_id(&self) -> i64 {
        match self {
            Chat::Private { id, .. } => *id,
            Chat::Group { id, .. } => *id,
            Chat::Supergroup { id, .. } => *id,
            Chat::Channel { id, .. } => *id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGES: &str = r#"
    {
  "ok": true,
  "result": [
    {
      "update_id": 159601961,
      "message": {
        "message_id": 468,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740484843,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601962,
      "message": {
        "message_id": 469,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740488786,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601963,
      "message": {
        "message_id": 470,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740488879,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601964,
      "message": {
        "message_id": 471,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740488881,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601965,
      "message": {
        "message_id": 476,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740490794,
        "text": "/frog how are you?",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601966,
      "message": {
        "message_id": 482,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4637523390,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740491073,
        "group_chat_created": true
      }
    },
    {
      "update_id": 159601967,
      "message": {
        "message_id": 483,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4637523390,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740491083,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601968,
      "message": {
        "message_id": 484,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": 1700048531,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "type": "private"
        },
        "date": 1740491106,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601969,
      "message": {
        "message_id": 490,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4676384907,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740494723,
        "group_chat_created": true
      }
    },
    {
      "update_id": 159601970,
      "message": {
        "message_id": 491,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4676384907,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740494728,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601971,
      "message": {
        "message_id": 502,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4676384907,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740530677,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    },
    {
      "update_id": 159601972,
      "message": {
        "message_id": 503,
        "from": {
          "id": 1700048531,
          "is_bot": false,
          "first_name": "Doug",
          "last_name": "Dimmadome",
          "username": "beek_en_donk",
          "language_code": "en"
        },
        "chat": {
          "id": -4676384907,
          "title": "Doug and FrogAI",
          "type": "group",
          "all_members_are_administrators": true
        },
        "date": 1740530682,
        "text": "/help",
        "entities": [{ "offset": 0, "length": 5, "type": "bot_command" }]
      }
    }
  ]
}
    "#;

    #[test]
    fn test_deserialize_messages() {
        let response: Response = serde_json::from_str(MESSAGES).expect("Failed to parse messages");

        // Verify the response was parsed correctly
        assert!(response.ok);
        assert_eq!(response.result.len(), 12);

        // Test a message with text and entities
        let message_with_text = &response.result[0].message;
        assert_eq!(message_with_text.message_id, 468);
        assert_eq!(message_with_text.text, Some("/help".to_string()));
        assert!(message_with_text.entities.is_some());
        assert_eq!(
            message_with_text.entities.as_ref().unwrap()[0].type_,
            "bot_command"
        );

        // Test a message with group_chat_created
        let message_with_group_chat = &response.result[5].message;
        assert_eq!(message_with_group_chat.message_id, 482);
        assert!(message_with_group_chat.group_chat_created.unwrap());
        assert_eq!(message_with_group_chat.text, None);

        // Test chat types
        match &response.result[0].message.chat {
            Chat::Private {
                id,
                first_name,
                last_name,
                username,
            } => {
                assert_eq!(*id, 1700048531);
                assert_eq!(first_name, "Doug");
                assert_eq!(last_name, &Some("Dimmadome".to_string()));
                assert_eq!(username, &Some("beek_en_donk".to_string()));
            }
            _ => panic!("Expected Private chat"),
        }

        match &response.result[5].message.chat {
            Chat::Group {
                id,
                title,
                all_members_are_administrators,
            } => {
                assert_eq!(*id, -4637523390);
                assert_eq!(title, "Doug and FrogAI");
                assert!(all_members_are_administrators.unwrap());
            }
            _ => panic!("Expected Group chat"),
        }
    }
}
