#[macro_export]
macro_rules! db_or_debug_and_return {
    ($s: ident, $msg: expr, $actor_err: expr, $mailbox_err: expr) => {
        $crate::actor_or_debug_and_return!($s, db, $msg, $actor_err, $mailbox_err)
    };
    ($s: ident, $msg: expr) => {
        $crate::actor_or_debug_and_return!($s, db, $msg)
    };
}

#[macro_export]
macro_rules! mail_or_debug_and_return {
    ($s: ident, $msg: expr, $actor_err: expr, $mailbox_err: expr) => {
        $crate::actor_or_debug_and_return!($s, mail, $msg, $actor_err, $mailbox_err)
    };
    ($s: ident, $msg: expr) => {
        $crate::actor_or_debug_and_return!($s, mail, $msg)
    };
}

#[macro_export]
macro_rules! actor_or_debug_and_return {
    ($s: ident, $actor: ident, $msg: expr, $actor_err: expr, $mailbox_err: expr) => {
        match block_on($s.$actor.send($msg)) {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                log::error!("{:?}", e);
                return $actor_err;
            }
            Err(e) => {
                log::error!("{:?}", e);
                return $mailbox_err;
            }
        }
    };
    ($s: ident, $actor: ident, $msg: expr) => {
        crate::actor_or_debug_and_return!($s, $actor, $msg, Ok(None), Ok(None))
    };
}

#[macro_export]
macro_rules! actor_or_debug_and_ignore {
    ($s: ident, $actor: ident, $msg: expr, $on_success: expr) => {
        match block_on($s.$actor.send($msg)) {
            Ok(Ok(r)) => {
                $on_success(r);
            }
            Ok(Err(e)) => {
                log::error!("{:?}", e);
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
    };
}

#[macro_export]
macro_rules! actor_or_debug_and_fallback {
    ($s: ident, $actor: ident, $msg: expr, $actor_err: expr, $mailbox_err: expr) => {
        match block_on($s.$actor.send($msg)) {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                log::error!("{:?}", e);
                $actor_err
            }
            Err(e) => {
                log::error!("{:?}", e);
                $mailbox_err
            }
        }
    };
}
