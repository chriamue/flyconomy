pub trait IdentityTrait: Sync + Send {
    // Returns the unique ID of the identity
    fn id(&self) -> String;

    // Returns the alias or display name of the identity
    fn alias(&self) -> String;

    // Sets the alias or display name of the identity
    fn set_alias(&mut self, alias: String);
}

#[derive(Clone, Debug)]
pub struct Identity {
    id: String,    // Default or cryptic ID, e.g., "0"
    alias: String, // Set by the player, e.g., "Pilot" or "Horst"
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            id: "0".to_string(),
            alias: "Pilot".to_string(),
        }
    }
}

impl IdentityTrait for Identity {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn alias(&self) -> String {
        self.alias.clone()
    }

    fn set_alias(&mut self, alias: String) {
        self.alias = alias;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_default() {
        let identity = Identity::default();

        // Test default values
        assert_eq!(identity.id(), "0");
        assert_eq!(identity.alias(), "Pilot");
    }

    #[test]
    fn test_identity_id() {
        let identity = Identity {
            id: "123".to_string(),
            alias: "John".to_string(),
        };

        // Test id method
        assert_eq!(identity.id(), "123");
    }

    #[test]
    fn test_identity_alias() {
        let identity = Identity {
            id: "123".to_string(),
            alias: "John".to_string(),
        };

        // Test alias method
        assert_eq!(identity.alias(), "John");
    }

    #[test]
    fn test_identity_set_alias() {
        let mut identity = Identity {
            id: "123".to_string(),
            alias: "John".to_string(),
        };

        // Test set_alias method
        identity.set_alias("Doe".to_string());
        assert_eq!(identity.alias(), "Doe");
    }
}
