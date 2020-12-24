#[macro_export]
macro_rules! db_pool {
    ($self: expr) => {
        &$self.pool.get().map_err(|e| {
            log::error!("{:?}", e);
            $crate::DatabaseError::DatabaseConnectionLost
        })?
    };
    ($self: expr, $pool: expr) => {
        &$pool.get().map_err(|e| {
            log::error!("{:?}", e);
            $crate::DatabaseError::DatabaseConnectionLost
        })?
    };
}

#[macro_export]
macro_rules! q {
    ($q: expr) => {{
        let q = $q;
        log::debug!(
            "{}",
            diesel::debug_query::<diesel::pg::Pg, _>(&q).to_string()
        );
        q
    }};
}

#[macro_export]
macro_rules! db_find {
    ($action: ident, $self: ident => $conn: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, $conn: &$crate::DbPooledConn) -> Result<$resource, crate::DatabaseError> {
            use crate::schema:: $schema ::dsl::*;
            let $self = self;
            $crate::q!($q)
                .first($conn)
                .map_err(|e| {
                    log::error!("{:?}", e);
                    $crate::DatabaseError::GenericFailure(
                        $crate::OperationError::LoadCollection,
                        $crate::ResourceKind::$resource,
                    )
                })
          }
        }

        impl actix::Message for $action {
            type Result = Result<$resource, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<$resource, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
              let $conn = $crate::db_pool!(self);
              msg.execute($conn)
            }
        }
    };
    ($action: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
      $crate::db_find! { $action, $self => conn => $schema => $q, $resource, $($field => $ty),+ }
    };
}

#[macro_export]
macro_rules! db_load {
    ($action: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, conn: &$crate::DbPooledConn) -> Result<Vec<$resource>, $crate::DatabaseError> {
            use crate::schema:: $schema ::dsl::*;
            let $self = self;
            $crate::q!($q)
                  .load(conn)
                  .map_err(|e| {
                      log::error!("{:?}", e);
                      $crate::DatabaseError::GenericFailure(
                          $crate::OperationError::LoadCollection,
                          $crate::ResourceKind::$resource,
                      )
                  })
          }
        }

        impl actix::Message for $action {
            type Result = Result<Vec<$resource>, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<Vec<$resource>, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
              let conn = $crate::db_pool!(self);

              msg.execute(conn)
            }
        }
    };
}

#[macro_export]
macro_rules! db_load_field {
    ($action: ident, $return_type: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, conn: &$crate::DbPooledConn) -> Result<Vec<$return_type>, $crate::DatabaseError> {
            use crate::schema:: $schema ::dsl::*;
            let $self = self;
            $crate::q!($q)
                  .load(conn)
                  .map_err(|e| {
                      log::error!("{:?}", e);
                      $crate::DatabaseError::GenericFailure(
                          $crate::OperationError::LoadCollection,
                          $crate::ResourceKind::$resource,
                      )
                  })
          }
        }

        impl actix::Message for $action {
            type Result = Result<Vec<$return_type>, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<Vec<$return_type>, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
              let conn = $crate::db_pool!(self);

              msg.execute(conn)
            }
        }
    };
}
#[macro_export]
macro_rules! db_create {
  ($action: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
      $crate::db_create_with_conn! { $action, $self => conn => $schema => $q, $resource, $($field => $ty),+ }
  }
}

#[macro_export]
macro_rules! db_create_with_conn {
    ($action: ident, $self: ident => $conn: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, $conn: &$crate::DbPooledConn) -> Result<$resource, crate::DatabaseError> {
            crate::Guard::new($conn)?.run(|_guard| {
              use crate::schema:: $schema ::dsl::*;
              let $self = self;
              $crate::q!($q)
                .get_result::<$resource>($conn)
                .map_err(|e| {
                    log::error!("{:?}", e);
                    $crate::DatabaseError::GenericFailure(
                        $crate::OperationError::Create,
                        $crate::ResourceKind::$resource,
                    )
                })
            })
          }
        }

        impl actix::Message for $action {
            type Result = Result<$resource, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<$resource, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
                let $conn = $crate::db_pool!(self);

                msg.execute($conn)
            }
        }
    };
}
#[macro_export]
macro_rules! db_update {
    ($action: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
      $crate::db_update_with_conn! { $action, $self => conn => $schema => $q, $resource, $($field => $ty),+ }
    };
}

#[macro_export]
macro_rules! db_update_with_conn {
    ($action: ident, $self: ident => $conn: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, $conn: &$crate::DbPooledConn) -> Result<$resource, crate::DatabaseError> {
            use crate::schema:: $schema ::dsl::*;
            let $self = self;
            $crate::q!($q)
                .get_result::<$resource>($conn)
                .map_err(|e| {
                    log::error!("{:?}", e);
                    $crate::DatabaseError::GenericFailure(
                        $crate::OperationError::Update,
                        $crate::ResourceKind::$resource,
                    )
                })
          }
        }

        impl actix::Message for $action {
            type Result = Result<$resource, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<$resource, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
                let $conn = $crate::db_pool!(self);

                msg.execute ( $conn )
            }
        }
    };
}

#[macro_export]
macro_rules! db_delete {
    ($action: ident, $self: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        $crate::db_delete_with_conn! { $action, $self => conn => $schema => $q, $resource, $($field => $ty),+ }
    };
}

#[macro_export]
macro_rules! db_delete_with_conn {
    ($action: ident, $self: ident => $conn: ident => $schema: ident => $q: expr, $resource: ident, $($field: ident => $ty: ty),+) => {
        pub struct $action {
          $(pub $field : $ty),+
        }

        impl $action {
          pub fn execute(self, $conn: &$crate::DbPooledConn) -> Result<usize, $crate::DatabaseError> {
            use $crate::schema:: $schema ::dsl::*;
            let $self = self;
            $crate::q!($q)
                .execute($conn)
                .map_err(|e| {
                    log::error!("{:?}", e);
                    $crate::DatabaseError::GenericFailure(
                        $crate::OperationError::Delete,
                        $crate::ResourceKind::$resource,
                    )
                })
          }
        }

        impl actix::Message for $action {
            type Result = Result<usize, $crate::DatabaseError>;
        }

        impl actix::Handler<$action> for $crate::DbExecutor {
            type Result = Result<usize, $crate::DatabaseError>;

            fn handle(&mut self, msg: $action, _ctx: &mut Self::Context) -> Self::Result {
                let $conn = $crate::db_pool!(self);

                msg.execute($conn)
            }
        }
    };
}
