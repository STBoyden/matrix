#![allow(clippy::pedantic)]
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

    trait Dot<I> {
        fn dot(self, matrix2: Self) -> Self;
        fn scalar_dot(self, multipiler: I) -> Self;
    };

    TokenStream::from(quote! {{
        #[derive(Debug, Eq, PartialEq)]
        pub struct Matrix([i32; #output_len]);

        // Prints the user inputted array in the same dimensions as the passed in
        // Looks like: [ 1,  2,  3,
        //               4,  5,  6,
        //               7,  8,  9,]

        impl std::fmt::Display for Matrix {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        // TODO: Panic message when index out of bounds is badd - plz fix
        impl<U> std::ops::Index<[U; 2]> for Matrix
        where
            U: std::convert::Into<usize> + std::marker::Copy,
            usize: std::ops::Mul<U, Output = usize> + std::ops::Add<U, Output = usize>,
        {
            // TODO: This has to be made more generic if the rest of the macro is made more generic
            type Output = i32;

            fn index(&self, idx: [U; 2]) -> &Self::Output {
                let index = #x_dim * idx[1] + idx[0];
                &self.0[index]
            }
        }

        // TODO: Broken - add product matrix type or something. help
        impl<U: std::ops::Mul<i32, Output = i32>> Dot<U> for Matrix {
            fn dot(self, matrix2: Self) -> Self {
                Matrix([1, 2, 3, 4, 5, 6, 7, 8, 9])
            }

            fn scalar_dot(self, multipiler: U) -> Self {
                let return_matrix = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
                Matrix(return_matrix)
            }
        }

        Matrix(#output_arr)
    }})
}
