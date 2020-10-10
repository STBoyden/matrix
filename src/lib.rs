use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod parse;
use parse::MatrixInput;

#[proc_macro]
pub fn matrix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MatrixInput);
    let output_len = input.dimensions.0 * input.dimensions.1;
    let output_arr = input.parsed_arr;

    let x_dim = input.dimensions.0;
    let y_dim = input.dimensions.1;

    TokenStream::from(quote! {{
        use std::fmt::{Display, Formatter, Result};
        #[derive(Debug)]
        pub struct Matrix([i32; #output_len]);

        // Prints the user inputted array in the same dimensions as the passed in
        impl Display for Matrix {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                for i in 0..#x_dim {
                    if i == 0 { write!(f, "[")? } else { write!(f, " ")? };
                    for j in 0..#y_dim {
                        write!(f, "{:>3},", self.0[#y_dim*i+j])?;
                    }
                    if i == #x_dim-1 { write!(f, "]")? } else { write!(f, "\n")? };
                }
                Ok(())
            }
        }

        Matrix(#output_arr)
    }})
}
