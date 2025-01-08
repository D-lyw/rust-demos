use chrono::{DateTime, Utc};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};

pub struct Storage {
    db: DB,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();

        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open(&opts, "tui-db")?;

        Ok(Storage { db })
    }

    pub fn get_session(&self, session_id: &str) -> Result<Session, Box<dyn std::error::Error>> {
        let session_data = self.db.get(session_id.as_bytes())?;
        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_slice(&data)?;
                Ok(session)
            }
            None => Err("Session not found".into()),
        }
    }

    pub fn save_session(&self, session: &Session) -> Result<(), Box<dyn std::error::Error>> {
        let session_data = serde_json::to_vec(session)?;
        self.db.put(session.id.as_bytes(), session_data)?;
        Ok(())
    }

    pub fn delete_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.delete(session_id.as_bytes())?;
        Ok(())
    }

    pub fn create_session(&self) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = Session {
            id: session_id,
            messages: Vec::new(),
        };
        self.save_session(&session)?;
        Ok(session_id)
    }

    pub fn add_message(
        &self,
        session_id: &str,
        user_input: &str,
        model_responses: Vec<(String, String)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut session = self.get_session(session_id)?;
        let order = session.messages.len() as u64 + 1;
        let timestamp = Utc::now();
        let message = Message {
            order,
            user_input: user_input.to_string(),
            model_responses: model_responses
                .into_iter()
                .map(|(model_name, response)| ModelResponse {
                    model_name,
                    response,
                })
                .collect(),
            timestamp,
        };
        session.messages.push(message);
        self.save_session(&session)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    id: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    order: u64,
    user_input: String,
    model_responses: Vec<ModelResponse>,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelResponse {
    model_name: String,
    response: String,
}
