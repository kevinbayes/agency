#[cfg(test)]
mod tests {
    use crate::a2a::AgentCard;
    use serde_json::json;

    #[test]
    fn test_agent_card_serialization() {
        let card = AgentCard {
            name: "Test Agent".to_string(),
            description: "A test agent".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&card).expect("Failed to serialize AgentCard");
        
        assert_eq!(json["name"], "Test Agent");
        assert_eq!(json["description"], "A test agent");
        assert_eq!(json["version"], "1.0.0");
    }
}
