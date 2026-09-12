use fhevm_engine_common::pg_pool::is_fatal_connection_error;

/// How a database error should be applied to a finite per-identity retry budget.
///
/// Fatal connection loss is handled separately by
/// [`is_fatal_connection_error`]: it still fails the worker task.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DbErrorClass {
    /// Deadlock, serialization failure, lock timeout, statement timeout, pool
    /// acquire timeout. Retry later without charging the identity's budget.
    Transient,
    /// Integrity/constraint (unique, check, FK, not-null). May be a race or a
    /// stuck invariant. Charge the finite retry budget.
    Integrity,
    /// Schema, decode, or data exception that will not heal. Exhaust the
    /// identity immediately.
    Definitive,
}

pub(crate) fn classify_db_error(error: &sqlx::Error) -> DbErrorClass {
    if is_fatal_connection_error(error) {
        // Callers must treat fatal errors as task failure before using this
        // classification. Mapping them here keeps the function total.
        return DbErrorClass::Definitive;
    }
    match error {
        sqlx::Error::PoolTimedOut => DbErrorClass::Transient,
        sqlx::Error::Database(database) => classify_sqlstate(database.code().as_deref()),
        sqlx::Error::RowNotFound
        | sqlx::Error::TypeNotFound { .. }
        | sqlx::Error::ColumnIndexOutOfBounds { .. }
        | sqlx::Error::ColumnNotFound(_)
        | sqlx::Error::ColumnDecode { .. }
        | sqlx::Error::Encode(_)
        | sqlx::Error::Decode(_)
        | sqlx::Error::Configuration(_) => DbErrorClass::Definitive,
        _ => DbErrorClass::Integrity,
    }
}

fn classify_sqlstate(code: Option<&str>) -> DbErrorClass {
    let Some(code) = code.filter(|code| !code.is_empty()) else {
        return DbErrorClass::Integrity;
    };
    if is_transient_sqlstate(code) {
        DbErrorClass::Transient
    } else if code.starts_with("23") {
        DbErrorClass::Integrity
    } else if code.starts_with("22") || code.starts_with("42") || code.starts_with("0A") {
        DbErrorClass::Definitive
    } else {
        DbErrorClass::Integrity
    }
}

fn is_transient_sqlstate(code: &str) -> bool {
    code.starts_with("40")
        || matches!(
            code,
            "55P03" // lock_not_available
                | "57014" // query_canceled / statement_timeout
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_sqlstates() {
        assert_eq!(classify_sqlstate(Some("40001")), DbErrorClass::Transient);
        assert_eq!(classify_sqlstate(Some("40P01")), DbErrorClass::Transient);
        assert_eq!(classify_sqlstate(Some("55P03")), DbErrorClass::Transient);
        assert_eq!(classify_sqlstate(Some("57014")), DbErrorClass::Transient);
        assert_eq!(classify_sqlstate(Some("23505")), DbErrorClass::Integrity);
        assert_eq!(classify_sqlstate(Some("23503")), DbErrorClass::Integrity);
        assert_eq!(classify_sqlstate(Some("23514")), DbErrorClass::Integrity);
        assert_eq!(classify_sqlstate(Some("23502")), DbErrorClass::Integrity);
        assert_eq!(classify_sqlstate(Some("42P01")), DbErrorClass::Definitive);
        assert_eq!(classify_sqlstate(Some("22P02")), DbErrorClass::Definitive);
        assert_eq!(classify_sqlstate(Some("0A000")), DbErrorClass::Definitive);
        assert_eq!(classify_sqlstate(Some("XX000")), DbErrorClass::Integrity);
        assert_eq!(classify_sqlstate(None), DbErrorClass::Integrity);
    }

    #[test]
    fn classifies_sqlx_variants() {
        assert_eq!(
            classify_db_error(&sqlx::Error::PoolTimedOut),
            DbErrorClass::Transient
        );
        assert_eq!(
            classify_db_error(&sqlx::Error::RowNotFound),
            DbErrorClass::Definitive
        );
        assert_eq!(
            classify_db_error(&sqlx::Error::ColumnNotFound("x".into())),
            DbErrorClass::Definitive
        );
        assert_eq!(
            classify_db_error(&sqlx::Error::PoolClosed),
            DbErrorClass::Definitive
        );
    }
}
