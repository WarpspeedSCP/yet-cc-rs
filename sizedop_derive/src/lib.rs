use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(SizedOpcode)]
pub fn sized_opcode_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_sized_opcode_macro(&ast)
}

fn impl_sized_opcode_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let name_int = name
    .to_string()
    .replace(|ch: char| !ch.is_digit(10), "")
    .parse::<usize>()
    .unwrap_or_default();
  let name_str = name.to_string();
  let mut chars = name_str.chars();
  let first_char = chars.next().unwrap();
  let second_char = chars.next().unwrap();

  let name_int = match first_char {
    'J' => name_int + 4,
    'S' => {
      if second_char == 'i' {
        0 // Single byte opcode.
      } else {
        4
      }
    }
    'C' => 6,
    'L' => 4, // long jump
    'D' => 4, // direct jump
    _ => name_int,
  };

  let size_fn =
    if (first_char == 'S' && second_char != 'i') || first_char == 'O' || first_char == 'C' {
      quote! {
        fn size(&self) -> usize {
          self.size
        }
      }
    } else {
      // A custom size function will be required in this case.
      quote! {}
    };

  let gen = quote! {
      impl SizedOpcode for #name {
          const BASE_SIZE: usize = #name_int;

          #size_fn
      }
  };
  gen.into()
}
