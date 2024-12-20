{{#include error_handling.rs}}
# struct Arbitrary;
#
# impl Arbitrary {
#     fn builder() -> Self {
#         Self
#     }
#
#     fn bar(self, _: Self) -> Self {
#         Self
#     }
#
#     fn build(self) -> Result<Self> {
#         Ok(Self)
#     }
# }
#
# struct Event {
#     response: Option<()>,
# }
#
# struct File {
#     name: (),
#     path: (),
# }
