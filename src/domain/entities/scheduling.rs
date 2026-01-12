use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchedulingInfo {
    pub time_from: Option<String>,
    pub time_to: Option<String>,
    pub days: Option<String>,
    pub weekdays: Option<String>,
    pub months: Vec<String>,
    pub days_calendar: Option<String>,
    pub weeks_calendar: Option<String>,
    pub conf_calendar: Option<String>,
    pub cyclic_interval: Option<String>,
    pub cyclic_times: Option<String>,
    pub max_wait: Option<i32>,
    pub max_rerun: Option<i32>,
}

impl SchedulingInfo {
    pub fn new() -> Self {
        Self {
            time_from: None,
            time_to: None,
            days: None,
            weekdays: None,
            months: Vec::new(),
            days_calendar: None,
            weeks_calendar: None,
            conf_calendar: None,
            cyclic_interval: None,
            cyclic_times: None,
            max_wait: None,
            max_rerun: None,
        }
    }

    pub fn has_calendar(&self) -> bool {
        self.days_calendar.is_some() 
            || self.weeks_calendar.is_some() 
            || self.conf_calendar.is_some()
    }

    pub fn has_time_window(&self) -> bool {
        self.time_from.is_some() && self.time_to.is_some()
    }

    pub fn is_cyclic(&self) -> bool {
        self.cyclic_interval.is_some() || self.cyclic_times.is_some()
    }

    pub fn complexity(&self) -> usize {
        let mut score = 0;
        
        if self.has_calendar() {
            score += 3;
        }
        if self.has_time_window() {
            score += 1;
        }
        if self.is_cyclic() {
            score += 5;
        }
        if !self.months.is_empty() {
            score += 2;
        }
        if self.weekdays.is_some() {
            score += 1;
        }
        
        score
    }
}

impl Default for SchedulingInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scheduling_info() {
        let sched = SchedulingInfo::new();
        assert!(!sched.has_calendar());
        assert!(!sched.has_time_window());
        assert!(!sched.is_cyclic());
    }

    #[test]
    fn test_scheduling_complexity() {
        let mut sched = SchedulingInfo::new();
        assert_eq!(sched.complexity(), 0);
        
        sched.days_calendar = Some("WORKDAYS".to_string());
        assert_eq!(sched.complexity(), 3);
        
        sched.cyclic_interval = Some("00:15".to_string());
        assert_eq!(sched.complexity(), 8);
    }

    #[test]
    fn test_has_time_window() {
        let mut sched = SchedulingInfo::new();
        assert!(!sched.has_time_window());
        
        sched.time_from = Some("08:00".to_string());
        assert!(!sched.has_time_window());
        
        sched.time_to = Some("18:00".to_string());
        assert!(sched.has_time_window());
    }
}
