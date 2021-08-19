pub mod adaptors;
pub mod consumers;
pub mod factories;
pub mod manual_executor;
pub mod traits;
pub mod utility;

pub(crate) mod priv_sync;

mod immediate_scheduler;

pub mod test;

pub use consumers::start_detached::start_detached;
pub use consumers::sync_wait::sync_wait;
pub use immediate_scheduler::ImmediateScheduler;
pub use traits::SenderExt;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<Option<T>, crate::Error>;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::factories::just::Just;
    use crate::traits::Scheduler;
    use crate::{sync_wait, SenderExt};

    #[test]
    fn just() {
        let res = sync_wait(Just::new(10));
        assert_eq!(res.unwrap(), 10);
    }

    #[test]
    fn map() {
        let res = Just::new(10).map(|x| 2 * x).sync_wait();
        assert_eq!(res.unwrap(), 20);
        let res = crate::factories::done().map(|_| 1).sync_wait();
        assert!(res.is_cancelled());
    }

    #[test]
    fn manual_executor_test() {
        let exec = Arc::new(crate::manual_executor::ManualExecutor::new());
        let runner = exec.runner();
        let res = exec
            .scheduler()
            .schedule()
            .map(|_| 2)
            .map(|x| 2 * x)
            .ensure_started();
        assert!(!res.is_complete());
        assert!(runner.run_one());
        assert_eq!(res.sync_wait().unwrap(), 4);
    }

    #[test]
    fn manual_exec_transfer() {
        let exec = crate::manual_executor::ManualExecutor::new();
        let exec2 = crate::manual_executor::ManualExecutor::new();

        let res = exec
            .scheduler()
            .schedule()
            .map(|_| 2)
            .map(|x| 2 * x)
            .transfer(exec2.scheduler())
            .ensure_started();
        assert!(!res.is_complete());
        assert!(exec.runner().run_one());
        assert!(!res.is_complete());
        assert!(exec2.runner().run_one());
        assert!(res.is_complete());
        assert_eq!(res.sync_wait().unwrap(), 4);
    }
}
