//! Scheduling entity module
//!
//! This module defines scheduling information for jobs including time windows,
//! calendars, cyclic execution, and other temporal constraints.

use serde::{Deserialize, Serialize};

/// Represents scheduling configuration for a job
///
/// SchedulingInfo contains all temporal constraints and execution timing
/// information for a job, including time windows, calendars, and cyclic settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchedulingInfo {
    /// Start time of execution window (e.g., "08:00")
    pub time_from: Option<String>,
    /// End time of execution window (e.g., "18:00")
    pub time_to: Option<String>,
    /// Specific days of month for execution
    pub days: Option<String>,
    /// Specific weekdays for execution
    pub weekdays: Option<String>,
    /// Specific months for execution
    pub months: Vec<String>,
    /// Calendar for day-based scheduling
    pub days_calendar: Option<String>,
    /// Calendar for week-based scheduling
    pub weeks_calendar: Option<String>,
    /// Confirmation calendar
    pub conf_calendar: Option<String>,
    /// Interval for cyclic execution (e.g., "00:15" for every 15 minutes)
    pub cyclic_interval: Option<String>,
    /// Number of times to run cyclically
    pub cyclic_times: Option<String>,
    /// Maximum wait time in minutes
    pub max_wait: Option<i32>,
    /// Maximum number of reruns on failure
    pub max_rerun: Option<i32>,
    
    // Additional scheduling attributes from Control-M XML
    /// Shift
    pub shift: Option<String>,
    /// Shift number
    pub shift_num: Option<String>,
    /// Retroactive scheduling
    pub retro: Option<String>,
    /// Statistical calendar
    pub stat_cal: Option<String>,
    /// Date specification
    pub date: Option<String>,
    /// Days AND/OR logic
    pub days_and_or: Option<String>,
    /// Maximum days
    pub maxdays: Option<i32>,
    /// Maximum runs
    pub maxruns: Option<i32>,
    /// Auto archive
    pub autoarch: Option<String>,
    /// Confirmation required
    pub confirm: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Active from date
    pub active_from: Option<String>,
    /// Active till date
    pub active_till: Option<String>,
    /// Due out time
    pub due_out: Option<String>,
    /// Due out days offset
    pub due_out_daysoffset: Option<String>,
    /// From days offset
    pub from_daysoffset: Option<String>,
    /// To days offset
    pub to_daysoffset: Option<String>,
    /// Previous day
    pub prev_day: Option<String>,
    /// Adjust condition
    pub adjust_cond: Option<String>,
}

impl SchedulingInfo {
    /// Creates a new SchedulingInfo with default values
    ///
    /// # Returns
    ///
    /// A new SchedulingInfo instance with all fields set to None or empty
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
            shift: None,
            shift_num: None,
            retro: None,
            stat_cal: None,
            date: None,
            days_and_or: None,
            maxdays: None,
            maxruns: None,
            autoarch: None,
            confirm: None,
            timezone: None,
            active_from: None,
            active_till: None,
            due_out: None,
            due_out_daysoffset: None,
            from_daysoffset: None,
            to_daysoffset: None,
            prev_day: None,
            adjust_cond: None,
        }
    }

    /// Checks if this scheduling uses any calendar
    ///
    /// # Returns
    ///
    /// `true` if any calendar is configured, `false` otherwise
    pub fn has_calendar(&self) -> bool {
        self.days_calendar.is_some() 
            || self.weeks_calendar.is_some() 
            || self.conf_calendar.is_some()
    }

    /// Checks if this scheduling has a time window defined
    ///
    /// # Returns
    ///
    /// `true` if both time_from and time_to are set, `false` otherwise
    pub fn has_time_window(&self) -> bool {
        self.time_from.is_some() && self.time_to.is_some()
    }

    /// Checks if this scheduling is cyclic
    ///
    /// # Returns
    ///
    /// `true` if cyclic interval or times are set, `false` otherwise
    pub fn is_cyclic(&self) -> bool {
        self.cyclic_interval.is_some() || self.cyclic_times.is_some()
    }

    /// Calculates the complexity score of this scheduling configuration
    ///
    /// Complexity is based on the number and type of scheduling constraints.
    /// More complex scheduling (calendars, cyclic, etc.) results in higher scores.
    ///
    /// # Returns
    ///
    /// A complexity score (higher means more complex)
    pub fn complexity(&self) -> usize {
        let mut score = 0;
        
        if self.has_calendar() {
            score += 3; // Calendar-based scheduling is moderately complex
        }
        if self.has_time_window() {
            score += 1; // Time windows add minor complexity
        }
        if self.is_cyclic() {
            score += 5; // Cyclic execution is highly complex
        }
        if !self.months.is_empty() {
            score += 2; // Month restrictions add complexity
        }
        if self.weekdays.is_some() {
            score += 1; // Weekday restrictions add minor complexity
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
