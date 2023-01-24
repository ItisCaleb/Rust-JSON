use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Data::Struct};

#[proc_macro_derive(Serializable, attributes(exclude))]
pub fn serializable_derive(input: TokenStream)->TokenStream{
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let s = match ast.data {
        Struct(st)=>st,
        _=>{
            panic!("Only struct can be serialize!")
        }
    };
    let fields = s.fields;
    let serial = match fields {
        syn::Fields::Named(named)=>{
            let mut st = quote!();
            'outer: for field in named.named{
                for p in &field.attrs{
                    if p.path.is_ident("exclude"){
                        continue 'outer
                    }
                }
                let fname = field.to_owned().ident.unwrap();
                let key = fname.to_string();
                st.extend(quote! {
                    object.put_ele(#key,self.#fname.serialize());
                });
            }
            st
        },
        _=>{
            panic!("undefined");
        }
    };
    let gen = quote! {
        impl Serializable for #name{
            fn serialize(&self) -> Box<dyn rjson::JsonElement>{
                let mut object = rjson::JsonObject::new();
                #serial
                object
            }
        }
    };
    gen.into()
}
