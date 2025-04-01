use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct};

#[proc_macro_derive(SizedOpcode)]
pub fn sized_opcode_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_sized_opcode_macro(&ast)
}

fn gen_size_opcode_impl(struct_name: &str, data: &DataStruct) -> Vec<proc_macro2::TokenStream> {
  let mut quotes = vec![];
  for field in &data.fields {
    let field_name = field.ident.as_ref().unwrap();
    let field_name_str = field_name.to_string();

    match field_name_str.as_str() {
      "opcode" | "n_choices" | "padding" => {
        quotes.push(quote! {
          size += 1; // #field_name_str
        });
      }
      "opt_arg2" => {
        quotes.push(quote! {
          if let Some(value) = self.#field_name {
            size += 2; // #field_name_str
          }
        });
      }
      "arg1" | "arg2" | "arg3" | "arg4" | "arg5" | "arg6" | "arg7" | "arg8" |
      "target_script" | "comparison_value" | "count" => {
        quotes.push(quote! {
          size += 2; // #field_name_str
        });
      }
      "jump_address" => {
        quotes.push(quote! {
          size += 2; // #field_name_str
        });
        if struct_name != "LongJumpOpcode" {
          quotes.push(quote! {
            size += 2; // #field_name_str
          });
        }
      }
      "padding_end" => {
        quotes.push(quote! {
          if let Some(value) = self.#field_name {
            size += 1; // #field_name_str
          }
        });
      }
      "arms" => {
        quotes.push(quote! {
          for arm in &self.arms {
            size += arm.size();
          }
        });
      }
      "pre_header" | "header" => {
        quotes.push(quote! {
          size += self.#field_name.len(); // #field_name_str
        });
      }
      "choices" => {
        quotes.push(quote! {
          for choice in &self.choices {
            size += choice.size();
          }
        });
      }
      "unicode" => quotes.push(quote! {
        {
          use encoding_rs::SHIFT_JIS;
          size += if let Some(tl) = &self.translation {
            crate::util::encode_sjis(tl).len()
          } else {
            crate::util::encode_sjis(&self.unicode).len()
          } + 1
        }
      }),
      _ => {}
    }
  }
  quotes
}

fn impl_sized_opcode_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let name_str = name.to_string();

  let quotes = match &ast.data {
    Data::Struct(opcode_struct) => gen_size_opcode_impl(&name_str, &opcode_struct),
    _ => panic!("{name_str} is not an opcode struct!"),
  };

  quote! {
    impl SizedOpcode for #name {
      fn size(&self) -> usize {
        let mut size = 0;

        #(#quotes)*

        size
      }
    }
  }
  .into()
}
