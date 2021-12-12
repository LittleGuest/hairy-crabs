use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, DeriveInput, ItemFn};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码解析为语法树以便进行操作
    let ast = syn::parse2(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro(){
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn hma(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: AttributeArgs = syn::parse_macro_input!(attr);

    // for arg in ast{
    //     arg.
    // }

    let item = syn::parse2::<ItemFn>(item).unwrap();
    let ident = &item.sig.ident;
    println!("{:?}", ident);

    let expanded = quote! {
        #item
    };

    expanded.into()
}
