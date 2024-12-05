use core::panic;

use quote::quote;
use syn::{Data, DataStruct};

#[proc_macro_derive(BinarySerialize)]
pub fn binary_serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_binary_serialize_macro(&ast)
}

fn gen_serialize_opcode_impl(data: &DataStruct) -> Vec<proc_macro2::TokenStream> {
  let mut quotes = vec![];
  for field in &data.fields {
    let field_name = field.ident.as_ref().unwrap();
    let field_name_str = field_name.to_string();

    match field_name_str.as_str() {
      "opcode" | "n_choices" | "padding" => {
        quotes.push(quote! {
          output.push(self.#field_name);
        });
      }
      "opt_arg2" => {
        quotes.push(quote! {
          if let Some(value) = self.#field_name {
            output.extend(value.to_le_bytes());
          }
        });
      }
      "arg1" | "arg2" | "arg3" | "arg4" | "arg5" | "arg6" | "arg7" | "arg8" | "jump_address"
      | "target_script" | "comparison_value" | "count" => {
        quotes.push(quote! {
          output.extend(self.#field_name.to_le_bytes());
        });
      }
      "padding_end" => {
        quotes.push(quote! {
          output.extend(self.#field_name.iter());
        });
      }
      "arms" => {
        quotes.push(quote! {
          for arm in &self.arms {
            output.extend(arm.index.to_le_bytes());
            output.extend(arm.jump_address.to_le_bytes());
          }
        });
      }
      "pre_header" | "header" => {
        quotes.push(quote! {
          output.extend(self.#field_name);
        });
      }
      "choices" => {
        quotes.push(quote! {
          {
            use encoding_rs::SHIFT_JIS;
            for choice in &self.choices {
              output.extend(choice.header);
              let res = if let Some(tl) = &choice.translation {
                SHIFT_JIS.encode(tl.as_str()).0
              } else {
                SHIFT_JIS.encode(&choice.unicode).0
              };
              output.extend(res.as_ref());
              output.push(0u8);
            }
          }
        });
      }
      "unicode" => quotes.push(quote! {
        if self.translation.is_none() {
          use encoding_rs::SHIFT_JIS;
          output.extend(SHIFT_JIS.encode(&self.unicode).0.as_ref());
          output.push(0u8);
        }
        
      }),
      "translation" => quotes.push(quote! {
        if let Some(tl) = &self.translation {
          use encoding_rs::SHIFT_JIS;
          output.extend(SHIFT_JIS.encode(tl.as_str()).0.as_ref());
          output.push(0u8);
        }
      }),
      _ => {}
    }
  }
  quotes
}

fn impl_binary_serialize_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
  let name = &ast.ident;
  let name_str = name.to_string();

  let quotes = match &ast.data {
    Data::Struct(opcode_struct) => gen_serialize_opcode_impl(&opcode_struct),
    _ => panic!("{name_str} is not an opcode struct!"),
  };

  quote! {
    impl BinarySerialize for #name {
      fn binary_serialize(&self) -> Vec<u8> {
        let mut output = vec![];

        #(#quotes)*

        output
      }
    }
  }
  .into()
}
