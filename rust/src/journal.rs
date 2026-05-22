//! Journal reminder messages and lookup logic.

/// Standard reminder message for logging in to the journal system (typically sent at 11 AM).
pub const JOURNAL_LOGIN_REMINDER: &str = "Hi <users/all>, Please dont forget to login to Journal";

/// Standard reminder message for logging daily tasks in the journal (typically sent at 6 PM and 11 PM).
pub const JOURNAL_LOG_TASKS_REMINDER: &str = "Hi <users/all>, Please dont forget to log tasks in journal";

/// Selects the appropriate journal reminder message based on a job name or trigger description.
/// E.g. job name containing "login" or "11 AM" maps to the login reminder, otherwise default to task logging.
pub fn get_message(job_name: &str) -> &'static str {
    let lower = job_name.to_lowercase();
    if lower.contains("login") || lower.contains("11 am") || lower.contains("11am") {
        JOURNAL_LOGIN_REMINDER
    } else {
        JOURNAL_LOG_TASKS_REMINDER
    }
}
