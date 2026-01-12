use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    In,
    Out,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    pub name: String,
    pub condition_type: ConditionType,
    pub odate: Option<String>,
    pub and_or: Option<String>,
}

impl Condition {
    pub fn new_in(name: String) -> Self {
        Self {
            name,
            condition_type: ConditionType::In,
            odate: None,
            and_or: None,
        }
    }

    pub fn new_out(name: String) -> Self {
        Self {
            name,
            condition_type: ConditionType::Out,
            odate: None,
            and_or: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnCondition {
    pub stmt: Option<String>,
    pub code: Option<String>,
    pub pattern: Option<String>,
    pub actions: Vec<DoAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DoAction {
    Action(String),
    Condition { name: String, sign: Option<String> },
    ForceJob { name: String, table_name: Option<String> },
    Mail { dest: String, message: String },
    Shout { dest: String, message: String },
    SetVariable { name: String, value: String },
}

impl OnCondition {
    pub fn new() -> Self {
        Self {
            stmt: None,
            code: None,
            pattern: None,
            actions: Vec::new(),
        }
    }

    pub fn complexity(&self) -> usize {
        let mut score = 1;
        score += self.actions.len();
        if self.pattern.is_some() {
            score += 2;
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
