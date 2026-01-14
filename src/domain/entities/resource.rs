use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlResource {
    pub name: String,
    pub resource_type: Option<String>,
    pub on_fail: Option<String>,
}

impl ControlResource {
    pub fn new(name: String) -> Self {
        Self {
            name,
            resource_type: None,
            on_fail: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantitativeResource {
    pub name: String,
    pub quantity: i32,
    pub on_fail: Option<String>,
    pub on_ok: Option<String>,
}

impl QuantitativeResource {
    pub fn new(name: String, quantity: i32) -> Self {
        Self {
            name,
            quantity,
            on_fail: None,
            on_ok: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_resource() {
        let resource = ControlResource::new("DB_LOCK".to_string());
        assert_eq!(resource.name, "DB_LOCK");
    }

    #[test]
    fn test_quantitative_resource() {
        let resource = QuantitativeResource::new("CPU_POOL".to_string(), 5);
        assert_eq!(resource.name, "CPU_POOL");
        assert_eq!(resource.quantity, 5);
    }
}
