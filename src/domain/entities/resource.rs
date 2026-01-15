//! Resource entity module
//!
//! This module defines control and quantitative resources used for job synchronization.
//! Resources prevent conflicts and manage concurrent access to shared systems.

use serde::{Deserialize, Serialize};

/// Represents a control resource (mutex)
///
/// Control resources ensure exclusive access to a resource. Only one job
/// can hold a control resource at a time, preventing concurrent access conflicts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlResource {
    /// Name of the control resource
    pub name: String,
    /// Type of resource (optional classification)
    pub resource_type: Option<String>,
    /// Action to take if resource acquisition fails
    pub on_fail: Option<String>,
}

impl ControlResource {
    /// Creates a new control resource
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the control resource
    ///
    /// # Returns
    ///
    /// A new ControlResource instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            resource_type: None,
            on_fail: None,
        }
    }
}

/// Represents a quantitative resource (semaphore)
///
/// Quantitative resources allow controlled concurrent access with a maximum
/// number of simultaneous users. Useful for limiting parallel job execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantitativeResource {
    /// Name of the quantitative resource
    pub name: String,
    /// Number of resource units required
    pub quantity: i32,
    /// Action to take if resource acquisition fails
    pub on_fail: Option<String>,
    /// Action to take when resource is successfully acquired
    pub on_ok: Option<String>,
}

impl QuantitativeResource {
    /// Creates a new quantitative resource
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the quantitative resource
    /// * `quantity` - Number of resource units required
    ///
    /// # Returns
    ///
    /// A new QuantitativeResource instance
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
