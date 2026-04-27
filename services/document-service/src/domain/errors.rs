// DomainError variants:
//   PageNotFound(String)
//   BlockNotFound(String)
//   VersionConflict { expected: i32, actual: i32 }
//   Unauthorized
//   Internal(#[source] anyhow::Error)
//
// pub type Result<T> = std::result::Result<T, DomainError>
