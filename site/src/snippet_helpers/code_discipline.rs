{{#include error_handling.rs}}
#
# struct Arbitrary;
# impl Arbitrary {
#     fn new() -> Self {
#         Self
#     }
#
#     fn something_else(self) -> Self {
#         self
#     }
#
#     fn another_thing(self) -> Self {
#         self
#     }
#
#     fn chained_with_something_else(self) -> Result<Self> {
#         Ok(self)
#     }
#
#     fn format_as(self, fmt: &str) -> String {
#         String::new()
#     }
# }
#
# fn some_long_computation() -> Result<Arbitrary> {
#     Ok(Arbitrary)
# }
#
# fn some_other_long_computation() -> Arbitrary {
#     Arbitrary
# }
#
# struct Input;
# trait Responder {
#     type Response;
#     type Err;
#     fn respond(&self, input: Input) -> std::result::Result<Self::Response, Self::Err>;
# }
#
#
# struct Message;
#
# impl Message {
#     fn text(&self) {}
# }
#
#
# struct Log {
#     log_file_path: String,
# }
#
# impl Log {
#     async fn transmit_log(&self, _: &str) -> Result<usize> {
#         Ok(0)
#     }
# }
