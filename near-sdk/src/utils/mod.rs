use crate::{env, AccountId, PromiseResult};

#[macro_export]
macro_rules! log {
    ($arg:tt) => {
        crate::env::log($arg.as_bytes())
    };
    ($($arg:tt)*) => {
        crate::env::log(format!($($arg)*).as_bytes())
    };
}

/// Assert that predecessor_account_id == current_account_id, meaning contract called itself.
pub fn assert_self() {
    assert_eq!(env::predecessor_account_id(), env::current_account_id());
}

/// Returns true if promise was successful.
/// Fails if called outside a callback that received 1 promise result.
pub fn is_promise_success() -> bool {
    promise_result_as_success().is_some()
}

/// Returns the result of the promise if successful. Otherwise returns None.
/// Fails if called outside a callback that received 1 promise result.
pub fn promise_result_as_success() -> Option<Vec<u8>> {
    assert_eq!(env::promise_results_count(), 1, "Contract expected a result on the callback");
    match env::promise_result(0) {
        PromiseResult::Successful(result) => Some(result),
        _ => None,
    }
}

/// Used in the simulation code generator from near_sdk.
#[derive(Debug)]
pub struct PendingContractTx {
    pub receiver_id: AccountId,
    pub method: String,
    pub args: Vec<u8>,
    pub is_view: bool,
}

impl PendingContractTx {
    pub fn new(receiver_id: &str, method: &str, args: serde_json::Value, is_view: bool) -> Self {
        Self {
            receiver_id: receiver_id.to_string(),
            method: method.to_string(),
            args: args.to_string().into_bytes(),
            is_view,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{get_logs, test_env};

    #[test]
    fn test_log_simple() {
        test_env::setup();
        log!("hello");

        assert_eq!(get_logs(), vec!["hello".to_string()]);
    }

    #[test]
    fn test_log_format() {
        test_env::setup();
        log!("hello {} ({})", "user_name", 25);

        assert_eq!(get_logs(), vec!["hello user_name (25)".to_string()]);
    }
}
