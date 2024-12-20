{{#include error_handling.rs}}
# struct Arbitrary;
#
# impl Arbitrary {
#     fn is_valid(&self) -> bool {
#         true
#     }
# }
#
# impl std::ops::Add for Arbitrary {
#     type Output = Self;
#
#     fn add(self, _: Self) -> Self {
#         Self
#     }
# }
#
# impl std::fmt::Display for Arbitrary {
#     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
#         write!(f, "Arbitrary")
#     }
# }
#
# fn foo() -> Arbitrary {
#     Arbitrary
# }
# fn bar() -> Arbitrary {
#     Arbitrary
# }
# fn baz() -> Arbitrary {
#     Arbitrary
# }
