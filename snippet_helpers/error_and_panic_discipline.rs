{{#include error_handling.rs}}
#
# mod url {
#     pub struct Url;
#
#     impl Url {
#         pub fn parse(_: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
#             Ok(Self)
#         }
#     }
# }
