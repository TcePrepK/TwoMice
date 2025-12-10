#[macro_export]
macro_rules! db_call {
    // -----------------------------------
    // Handle no bindings
    // -----------------------------------
    (
        pool  = $pool:expr,
        query = $query:expr,
        error = $error:ty,
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [],
            errors = $error
        )
    }};

    // -----------------------------------
    // Handle no bindings
    // -----------------------------------
    (
        pool  = $pool:expr,
        query = OPTIONAL $query:expr,
        error = $error:ty,
    ) => {{
        db_call!(
            pool   = $pool,
            query  = OPTIONAL $query,
            binds  = [],
            errors = $error
        )
    }};

    // -----------------------------------
    // Main solution with every parameter
    // -----------------------------------
    (
        pool  = $pool:expr,
        query = $query:expr,
        binds = [$($param:expr),* $(,)?],
        error = $error:ty
    ) => {{
        // Handle the bindings
        let mut query = $query;
        $( query = query.bind($param); )*

        // Handle the fetching and error mapping
        query.fetch_one($pool)
            .await
            .map_err(|err: sqlx::Error| {
                if let sqlx::Error::Database(db_err) = &err && let Some(code) = db_err.code().as_deref() {
                    // Cast it to the trait, then use the defined "from_code" function instead.
                    let mapped = <$error as $crate::easy_db_error::DbErrorTrait>::from_code(code);

                    // If `from_code` returned Unexpected, then log it
                    if <_ as $crate::easy_db_error::DbErrorTrait>::is_unexpected(&mapped) {
                        log::error!("UNEXPECTED SQLx ERROR (unmapped CODE {code}): {err:?}");
                    }

                    return mapped;
                }

                // If nothing fits, fallback to the unexpected error.
                log::error!("UNEXPECTED SQLx ERROR: {err:?}");
                <$error as $crate::easy_db_error::DbErrorTrait>::unexpected(err)
            })
    }};

    // --------------------------------------------
    // Main optional solution with every parameter
    // --------------------------------------------
    (
        pool  = $pool:expr,
        query = OPTIONAL $query:expr,
        binds = [$($param:expr),* $(,)?],
        error = $error:ty,
    ) => {{
        // Handle the bindings
        let mut query = $query;
        $( query = query.bind($param); )*

        // Handle the fetching and error mapping
        query.fetch_optional($pool)
            .await
            .map_err(|err: sqlx::Error| {
                if let sqlx::Error::Database(db_err) = &err && let Some(code) = db_err.code().as_deref() {
                    // Cast it to the trait, then use the defined "from_code" function instead.
                    let mapped = <$error as $crate::easy_db_error::DbErrorTrait>::from_code(code);

                    // If `from_code` returned Unexpected, then log it
                    if <_ as $crate::easy_db_error::DbErrorTrait>::is_unexpected(&mapped) {
                        log::error!("UNEXPECTED SQLx ERROR (unmapped CODE {code}): {err:?}");
                    }

                    return mapped;
                }

                // If nothing fits, fallback to the unexpected error.
                log::error!("UNEXPECTED SQLx ERROR: {err:?}");
                <$error as $crate::easy_db_error::DbErrorTrait>::unexpected(err)
            })
    }}
}
