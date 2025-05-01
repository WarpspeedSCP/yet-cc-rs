use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Opcodelike)]
pub fn opcodelike_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_opcodelike_macro(&ast)
}

fn impl_opcodelike_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;

  quote! {
    impl Opcodelike for #name {
      fn address(&self) -> u32 {
        self.address
      }

      fn opcode(&self) -> u8 {
        self.opcode
      }

      fn actual_address(&self) -> u32 {
        self.actual_address
      }

      fn set_actual_address(&mut self, new_addr: u32) {
        self.actual_address = new_addr;
      }
    }
  }
  .into()
}
