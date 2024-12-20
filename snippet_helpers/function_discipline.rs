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

mod some_crate {
    pub struct Foo;

    impl Foo {
        pub fn builder() -> FooBuilder {
            FooBuilder
        }
    }

    pub struct FooBuilder;

    impl FooBuilder {
        pub fn new() -> Self {
            Self
        }

        pub fn foo(self, _: &'static str) -> Self {
            self
        }

        pub fn bar(self, _: &'static str) -> Self {
            self
        }

        pub fn build(self) -> std::result::Result<Foo, Box<dyn std::error::Error>> {
            Ok(Foo)
        }
    }
}
