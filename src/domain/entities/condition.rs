//! Condition entity module
//!
//! This module defines conditions used for job dependencies and event handling.
//! Conditions are the primary mechanism for job orchestration in Control-M.

use serde::{Deserialize, Serialize};

/// Type of condition (input or output)
///
/// Input conditions are prerequisites that must be satisfied before a job runs.
/// Output conditions are set by a job upon completion to signal downstream jobs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    /// Input condition - must be satisfied before job execution
    In,
    /// Output condition - set by job upon completion
    Out,
}

/// Represents a job condition (input or output)
///
/// Conditions are used to create dependencies between jobs. A job waits for
/// its input conditions to be satisfied before running, and sets output
/// conditions when it completes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    /// Name of the condition
    pub name: String,
    /// Type of condition (In or Out)
    pub condition_type: ConditionType,
    /// Order date for the condition (ODATE)
    pub odate: Option<String>,
    /// Logical operator for multiple conditions (AND/OR)
    pub and_or: Option<String>,
}

impl Condition {
    /// Creates a new input condition
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the condition
    ///
    /// # Returns
    ///
    /// A new Condition instance of type In
    pub fn new_in(name: String) -> Self {
        Self {
            name,
            condition_type: ConditionType::In,
            odate: None,
            and_or: None,
        }
    }

    /// Creates a new output condition
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the condition
    ///
    /// # Returns
    ///
    /// A new Condition instance of type Out
    pub fn new_out(name: String) -> Self {
        Self {
            name,
            condition_type: ConditionType::Out,
            odate: None,
            and_or: None,
        }
    }
}

/// Represents an event-based condition with associated actions
///
/// OnConditions trigger actions based on job events (completion, failure, etc.)
/// They can execute various actions like setting conditions, sending notifications,
/// or forcing other jobs to run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnCondition {
    /// Statement or trigger condition
    pub stmt: Option<String>,
    /// Return code or status code to match
    pub code: Option<String>,
    /// Pattern to match in job output
    pub pattern: Option<String>,
    /// List of actions to execute when condition is met
    pub actions: Vec<DoAction>,
}

/// Actions that can be performed when an OnCondition is triggered
///
/// These actions allow jobs to interact with the Control-M environment
/// by setting conditions, forcing jobs, sending notifications, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DoAction {
    /// Generic action with a command string
    Action(String),
    /// Set or delete a condition
    Condition { name: String, sign: Option<String> },
    /// Force another job to run
    ForceJob { name: String, table_name: Option<String> },
    /// Send an email notification
    Mail { dest: String, message: String },
    /// Send a shout message (alert)
    Shout { dest: String, message: String },
    /// Set a variable value
    SetVariable { name: String, value: String },
}

impl OnCondition {
    /// Creates a new OnCondition with default values
    ///
    /// # Returns
    ///
    /// A new OnCondition instance with empty fields
    pub fn new() -> Self {
        Self {
            stmt: None,
            code: None,
            pattern: None,
            actions: Vec::new(),
        }
    }

    /// Calculates the complexity score of this OnCondition
    ///
    /// Complexity is based on the number of actions and presence of patterns.
    /// More actions and pattern matching increase complexity.
    ///
    /// # Returns
    ///
    /// A complexity score (higher means more complex)
    pub fn complexity(&self) -> usize {
        let mut score = 1; // Base complexity
        score += self.actions.len(); // Each action adds complexity
        if self.pattern.is_some() {
            score += 2; // Pattern matching adds significant complexity
        }
        score
    }
}

impl Default for OnCondition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_in_condition() {
        let cond = Condition::new_in("TEST_COND".to_string());
        assert_eq!(cond.name, "TEST_COND");
        assert_eq!(cond.condition_type, ConditionType::In);
    }

    #[test]
    fn test_on_condition_complexity() {
        let mut on_cond = OnCondition::new();
        assert_eq!(on_cond.complexity(), 1);
        
        on_cond.actions.push(DoAction::Action("OK".to_string()));
        on_cond.actions.push(DoAction::Action("NOTOK".to_string()));
        assert_eq!(on_cond.complexity(), 3);
        
        on_cond.pattern = Some("ERROR".to_string());
        assert_eq!(on_cond.complexity(), 5);
    }
}
