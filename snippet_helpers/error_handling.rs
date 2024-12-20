# type Result<T> = std::result::Result<T, Error>;
#
# enum Error {
#     Invalid,
#     NetworkUnavailable,
#     MalformedEnvUrl {
#         env_var: &'static str,
#         source: Box<dyn std::error::Error>,
#     },
#     Unsupported,
#     Unknown(Box<dyn std::error::Error>),
# }
#
# impl<E> From<E> for Error
# where
#     E: std::error::Error + 'static,
# {
#     fn from(err: E) -> Self {
#         Error::Unknown(Box::new(err))
#     }
# }
