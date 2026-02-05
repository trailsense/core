#[macro_export]
macro_rules! db_tx {
    ($pool:expr, |$tx:ident| $body:block) => {{
        use $crate::common::database::run_in_transaction;
        run_in_transaction($pool, |$tx| Box::pin(async move $body))
    }};
}
