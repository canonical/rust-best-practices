# type Result<T> = std::result::Result<T, Error>;
#
# enum Error {
#     Invalid,
#     NetworkUnavailable,
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
