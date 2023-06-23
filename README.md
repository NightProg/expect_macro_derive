# expect_macro_derive
This crate is used to generate a expected method for each variant of an enum.

# Usage

 this derive macro is used to generate a method for each variant of an enum.

 he method will be named `expect_{variant_name}` and will return a Option.
 if it matches the pattern it will return Some with the fields of the variant.
 otherwise it will return None.

  # Attributes

 ## `#[panic]`

 if this attribute is present on a variant, the generated method will panic instead of returning None.
 Note: the enum need to implement Debug.

 # Example

 ```rust
 use expect_macro::Expect;

 #[derive(Debug, Expect)]
 enum Foo {
    #[panic] Bar { a: i32, b: i32 },
     Baz(i32, i32),
     Qux,
 }

 fn main() {
    let bar = Foo::Bar { a: 1, b: 2 };
    let baz = Foo::Baz(1, 2);
    let qux = Foo::Qux;
    let (a, b) = bar.expect_bar(1, 2);
    let opt: Option<(i32, i32)> = baz.expect_baz(1, 2);
    assert_eq!(qux.expect_qux(), Some(()));
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(opt, Some((1, 2)));
 }
 ```
 # License

 Licensed under either of MIT license or Apache License, Version 2.0 at your option.

 Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

 # Contribution

 You can contribute to this project in many forms:

 - Report bugs and make suggestions on [Github](https://github.com/NightProg/expect_macro_rs/)
 - Submit pull requests with new features or bug fixes
 - Star this project on [Github](https://github.com/NightProg/expect_macro_rs/)
 - And more!

 
