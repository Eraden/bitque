#[macro_export]
macro_rules! query_db_or_print {
    ($s:expr,$msg:expr) => {
        match block_on($s.db.send($msg)) {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                log::error!("{:?}", e);
                return Ok(None);
            }
            Err(e) => {
                log::error!("{}", e);
                return Ok(None);
            }
        }
    };
}
