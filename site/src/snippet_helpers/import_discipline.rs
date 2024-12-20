{{#include error_handling.rs}}
# 
# mod some_crate {
#     pub struct SpecificItem1;
#     pub struct SpecificItem2;
# }
# 
# mod some_other_crate {
#     pub mod prelude {}
#     pub struct SpecificItem3;
# }
# 
# mod another_crate {
#     pub enum SomeEnum {
#         Variant1,
#         Variant2,
#     }
# }
# 
# mod allocative {
#     pub struct Allocative;
# }
# 
# mod derive_more {
#     pub struct Display;
# }
# 
# mod starlark {
#     pub mod environment {
#         pub struct FrozenModule;
#         pub struct Module;
#     }
# 
#     pub mod eval {
#         pub struct Evaluator;
#     }
# 
#     pub mod values {
#         pub struct AllocValue;
#         pub struct Freeze;
#         pub struct ProvidesStaticType;
#         pub struct StarlarkValue;
#         pub struct ValueLike;
#     }
# }
# 
# mod starlark_derive {
#     pub fn starlark_value() {}
#     pub struct NoSerialize;
#     pub struct Trace;
# }
