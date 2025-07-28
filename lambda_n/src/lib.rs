//! A Rust port of Prof. Shriram Krishnamurthi's `lambda-n` language:
//! https://github.com/shriram/lambda-n

use quote::{ToTokens, format_ident, quote};
use syn::{
    Expr, ExprLit, Ident, Token, parse::Parse, punctuated::Punctuated, spanned::Spanned,
    token::Comma,
};

/// Creates a closure with a specified number of arguments.
///
/// The macro takes an integer `N` and a body expression, generating a closure
/// that accepts `N` arguments. The arguments are automatically named `__1`, `__2`, ..., `__N`
/// and are available for use within the body.
///
/// # Syntax
///
/// `lambda_n!([move] <integer> => <expression>)`
///
/// # Example
///
/// ```ignore
/// // Creates a closure that takes 3 arguments and adds them.
/// let add_three = lambda_n!(3 => __1 + __2 + __3);
/// assert_eq!(add_three(10, 20, 5), 35);
/// ```
#[proc_macro]
pub fn lambda_n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = syn::parse_macro_input!(input as LambdaN);
    quote!(#c).into()
}

/// Intermediate representation of the `lambda_n!` macro input.
struct LambdaN {
    /// Whether or not the closure should take ownership of its env
    should_move: bool,
    /// The number of arguments for the generated closure.
    n_args: u16,
    /// The expression that forms the body of the closure.
    body: syn::Expr,
}

impl Parse for LambdaN {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let should_move = input.peek(Token![move]);
        if should_move {
            input.parse::<Token![move]>()?;
        }
        let n_args = extract_integer(input.parse()?)?;
        _ = input.parse::<Token![=>]>()?;
        let body = input.parse()?;

        Ok(Self {
            should_move,
            n_args,
            body,
        })
    }
}

/// A helper function to parse a `syn::Expr` into a `u16`.
fn extract_integer(expr: Expr) -> syn::Result<u16> {
    if let Expr::Lit(ExprLit { lit, .. }) = &expr {
        match lit {
            syn::Lit::Int(lit_int) => lit_int.base10_parse(),
            _ => Err(syn::Error::new(expr.span(), "Expected a integer literal")),
        }
    } else {
        Err(syn::Error::new(expr.span(), "Expected a integer literal"))
    }
}

impl ToTokens for LambdaN {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let body = &self.body;
        let args = (1..=self.n_args).fold(Punctuated::<Ident, Comma>::new(), |mut acc, arg| {
            acc.push(format_ident!("__{}", arg));
            acc
        });

        if self.should_move {
            tokens.extend(quote!({ move |#args|  #body }));
        } else {
            tokens.extend(quote!({ |#args|  #body }));
        }
    }
}
