//! # TAO Module
//!
//! Layer 3: Encapsulation and message passing.

/// TAO message
#[derive(Debug, Clone)]
pub struct TaoMessage {
    pub sender: String,
    pub receiver: String,
    pub content: MessageContent,
    pub timestamp: u64,
}

/// Message content types
#[derive(Debug, Clone)]
pub enum MessageContent {
    Query(String),
    Response(String),
    Event(String),
    Command { action: String, args: Vec<String> },
}

/// TAO agent
#[derive(Debug)]
pub struct TaoAgent {
    pub name: String,
    inbox: Vec<TaoMessage>,
}

impl TaoAgent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inbox: Vec::new(),
        }
    }

    /// Send message
    pub fn send(&self, receiver: &str, content: MessageContent) -> TaoMessage {
        TaoMessage {
            sender: self.name.clone(),
            receiver: receiver.to_string(),
            content,
            timestamp: 0, // Would use actual timestamp
        }
    }

    /// Receive message
    pub fn receive(&mut self, msg: TaoMessage) {
        self.inbox.push(msg);
    }

    /// Process next message
    pub fn process_next(&mut self) -> Option<TaoMessage> {
        if self.inbox.is_empty() {
            None
        } else {
            Some(self.inbox.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tao_agent() {
        let agent = TaoAgent::new("grammar");
        let msg = agent.send("semantic", MessageContent::Query("analyze".to_string()));
        assert_eq!(msg.sender, "grammar");
        assert_eq!(msg.receiver, "semantic");
    }
}
