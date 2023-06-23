#![allow(unused_assignments)]

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

///
/// # Usage
///
/// this derive macro is used to generate a method for each variant of an enum.
///
/// he method will be named `expect_{variant_name}` and will return a Option.
/// if it matches the pattern it will return Some with the fields of the variant.
/// otherwise it will return None.
///
/// If the attribute `#[panic]` is present on the variant, the method will panic instead of returning None.
/// and the panic message will be the name of the variant.
///
/// # Example
///
/// ```rust
/// use expect_macro::Expect;
///
/// #[derive(Debug, Expect)]
/// enum Foo {
///    #[panic] Bar { a: i32, b: i32 },
///     Baz(i32, i32),
///     Qux,
/// }
///
/// fn main() {
///    let bar = Foo::Bar { a: 1, b: 2 };
///    let baz = Foo::Baz(1, 2);
///    let qux = Foo::Qux;
///    let (a, b) = bar.expect_bar(1, 2);
///    let opt: Option<(i32, i32)> = baz.expect_baz(1, 2);
///    assert_eq!(qux.expect_qux(), Some(()));
///    assert_eq!(a, 1);
///    assert_eq!(b, 2);
///    assert_eq!(opt, Some((1, 2)));
/// }
/// ```
///
/// # Attributes
///
/// ## `#[panic]`
///
/// if this attribute is present on a variant, the generated method will panic instead of returning None.
///
/// Note: the enum need to implement Debug.

#[proc_macro_derive(Expect, attributes(panic))]
pub fn expect_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name = derive_input.ident;
    let mut methods = Vec::new();


    match derive_input.data {
        Data::Enum(e) => {

            for variant in e.variants {
                let variant_name = variant.ident;

                let is_panic = variant.attrs.iter().any(|attr| {
                    attr.path().is_ident("panic")
                });

                let fn_name = format!("expect_{}", variant_name.to_string().to_lowercase());
                let fn_name = syn::Ident::new(&fn_name, name.span());

                let mut fields = vec![quote![]];
                let mut fields_names = vec![quote![]];
                let mut fields_ty= vec![quote![]];
                let mut pattern = quote![];
                let mut new = quote![];

                match variant.fields {
                    syn::Fields::Named(named) => {
                        fields = named.named.iter().map(|field| {
                            let name = field.ident.as_ref().expect("Expected field name");
                            let ty = &field.ty;
                            quote! {
                                #name: #ty
                            }
                        }).collect::<Vec<_>>();

                        let args = named.named.iter().map(|field| {
                            let name = field.ident.as_ref().expect("Expected field name");
                            quote! {
                                #name
                            }
                        }).collect::<Vec<_>>();

                        fields_names = named.named.iter().map(|field| {
                            let name = field.ident.as_ref().expect("Expected field name 1");
                            let name = syn::Ident::new(&name.to_string().to_lowercase(), name.span());
                            let name = syn::Ident::new(&format!("attr_{}", name), name.span());
                            quote! {
                                #name
                            }
                        }).collect::<Vec<_>>();

                        fields_ty = named.named.iter().map(|field| {
                            let ty = &field.ty;
                            quote! {
                                #ty
                            }
                        }).collect::<Vec<_>>();

                        pattern = quote! {
                            #name::#variant_name { #(#args: #fields_names),* } if #(#fields_names == #args)&&*
                        };

                        new = quote! {
                            #name::#variant_name { #(#args),* }
                        };

                    },
                    syn::Fields::Unnamed(unnamed) => {
                        let mut n = 0;
                        let mut names = Vec::new();
                        fields = unnamed.unnamed.iter().map(|field| {
                            let ty = &field.ty;
                            let name = syn::Ident::new(&format!("value_{}", n), name.span());
                            names.push(name.clone());
                            n += 1;
                            quote! {
                                #name: #ty
                            }
                        }).collect::<Vec<_>>();
                        n = 0;
                        fields_names = unnamed.unnamed.iter().map(|_| {
                            let name = syn::Ident::new(&format!("attr_{}", n), name.span());
                            n += 1;
                            quote! {
                                #name
                            }
                        }).collect::<Vec<_>>();

                        fields_ty = unnamed.unnamed.iter().map(|field| {
                            let ty = &field.ty;
                            quote! {
                                #ty
                            }
                        }).collect::<Vec<_>>();

                        pattern = quote! {
                            #name::#variant_name( #(#fields_names),* ) if #(#fields_names == #names)&&*
                        };

                        new = quote! {
                            #name::#variant_name( #(#names),* )
                        };

                    },
                    syn::Fields::Unit => {
                        pattern = quote! {
                            #name::#variant_name
                        };

                        new = quote! {
                            #name::#variant_name
                        };
                    }
                }
                if is_panic {
                    let method = quote! {
                                pub fn #fn_name(self, #(#fields),*) -> (#(#fields_ty),*) {
                                    match self {
                                        #pattern => (#(#fields_names),*),
                                        _ => panic!("Expected {:?} but got {:?}", #new, self)
                                    }
                                }
                            };
                    methods.push(method);
                    continue;
                } else {
                    let method = quote! {
                                pub fn #fn_name(self, #(#fields),*) -> Option<(#(#fields_ty),*)> {
                                    match self {
                                        #pattern => Some((#(#fields_names),*)),
                                        _ => None
                                    }
                                }
                            };
                    methods.push(method);
                }

            }

        },
        _ => panic!("Expect can only be derived for enums")
    }
    let tokens = quote! {
        impl #name {
            #(#methods)*
        }
    };

    tokens.into()
}

