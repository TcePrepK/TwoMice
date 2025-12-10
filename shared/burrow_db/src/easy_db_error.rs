pub trait DbErrorTrait: Sized {
    fn from_code(code: &str) -> Self;
    fn unexpected(err: sqlx::Error) -> Self;
    fn is_unexpected(&self) -> bool;
}

#[macro_export]
macro_rules! define_errors {
    (
        $name:ident {
            $(
                $variant:ident => $code:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name {
            $(
                $variant,
            )*
            Unexpected(String),
        }

        impl $crate::easy_db_error::DbErrorTrait for $name {
            fn from_code(code: &str) -> Self {
                match code {
                    $(
                        $code => $name::$variant,
                    )*
                    other => $name::Unexpected(other.to_string()),
                }
            }

            fn unexpected(err: sqlx::Error) -> Self {
                $name::Unexpected(err.to_string())
            }

            fn is_unexpected(&self) -> bool {
                matches!(self, Self::Unexpected(_))
            }
        }
    };
}
