use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{spanned::Spanned, Error, Expr, ExprArray, Lit, Result};

#[derive(Debug)]
pub(crate) struct MatrixInput {
    pub(crate) parsed_arr: proc_macro2::TokenStream,
    pub(crate) dimensions: (usize, usize),
}

impl Parse for MatrixInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let array = ExprArray::parse(input)?;
        let dims = check_array_length(&array)?;
        let parsed_arr = check_valid_input(&array)?;
        Ok(MatrixInput {
            parsed_arr,
            dimensions: dims,
        })
    }
}

// Ensures each array within the passed in 2d arrays are the same lenght then returns the
// dimensions of the matrix
pub(super) fn check_array_length(array: &ExprArray) -> Result<(usize, usize)> {
    // Takes in pointer to an expression and returns the ExprArray struct that it should be, if
    // it's not the expected ExprArray struct errors
    fn unwrap_array(expr: &Expr) -> Result<ExprArray> {
        if let Expr::Array(array) = expr {
            return Ok(array.clone());
        }
        Err(Error::new(expr.span(), "Expected 2d array"))
    }

    // Iterates through the passsed in array comparing the current element to the first sub arrays
    // length
    let base_length = (unwrap_array(&array.elems[0])?).elems.len();
    for i in 1..(&array).elems.len() {
        let cur_elem = unwrap_array(&array.elems[i])?;
        if cur_elem.elems.len() != base_length {
            return Err(Error::new(
                array.elems[i].span(),
                "All of the arrays within the matrix must be the same length.",
            ));
        }
    }
    if let Expr::Array(sub_array) = &array.elems[0] {
        return Ok((array.elems.len(), sub_array.elems.len()));
    }
    Err(Error::new(array.span(), "Expected a 2d array"))
}

// Iterates through the passed in 2d array ensuring that it is only a 2d array.
// Creates a token stream of a flattened array of the passed in 2d array
fn check_valid_input(array: &ExprArray) -> Result<proc_macro2::TokenStream> {
    let mut arr_out = proc_macro2::TokenStream::new();
    for sub_array in &array.elems {
        if let Expr::Array(sub_array) = sub_array {
            for elem in &sub_array.elems {
                if let Expr::Lit(lit) = elem {
                    if let Lit::Int(lit) = &lit.lit {
                        if arr_out.is_empty() {
                            arr_out = quote! { #lit }
                        } else {
                            arr_out = quote! { #arr_out, #lit }
                        }
                    } else {
                        return Err(Error::new(lit.lit.span(), "Expected int or float."));
                    };
                } else if let Expr::Array(arr) = elem {
                    return Err(Error::new(arr.span(), "Expected a 2d array"));
                } else {
                    return Err(Error::new(
                        elem.span(),
                        "Expected integer or float literal.",
                    ));
                }
            }
        } else {
            return Err(Error::new(sub_array.span(), "Expected a 2d array."));
        }
    }
    Ok(quote! {[#arr_out]})
}
