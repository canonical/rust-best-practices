{{#include error_handling.rs}}
#
# struct Arbitrary;
#
# impl Arbitrary {
#     fn new() -> Self {
#         Self
#     }
#
#     fn builder() -> Self {
#         Self
#     }
#
#     fn foo(self, _: &str) -> Self {
#         self
#     }
#
#     fn bar(self, _: &str) -> Self {
#         self
#     }
#
#     fn build(self) -> Result<Self> {
#         Ok(self)
#     }
# }
