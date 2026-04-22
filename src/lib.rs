mod table_input;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
use table_input::TableInput;

/// ```rust, ignore
/// table!{
///     /// You can also add doc comments and attributes.
///     /// (If you need to add attributes to the macro, this is the only way.)
///     #[macro_export]
///     macro table_name [
///         [row input goes here]
///         [as many rows as you want]
///         [use [] for rows always]
///     ]
/// }
/// ```
/// This will create a macro like this:
/// ```rust, ignore
/// #[doc = "You can also add doc comments and attributes.\n(If you need to add attributes to the macro, this is the only way.)"]
/// #[macro_export]
/// macro_rules! table_name {
///     () => {
///         {row input goes here}
///         {as many rows as you want}
///         {use [] for rows always}
///     };
///     (foreach($___macro_callback:path)) => {
///         $___macro_callback!{ row input goes here }
///         $___macro_callback!{ as many rows as you want }
///         $___macro_callback!{ use [] for rows always }
///     };
///     ($___macro_callback:path) => {
///         $___macro_callback!{
///             {row input goes here}
///             {as many rows as you want}
///             {use [] for rows always}
///         }
///     };
/// }
/// ```
/// The created macro can be called like so:
/// ```rust, ignore
/// table_name!();
/// // or
/// table_name!(path_to::other_macro);
/// // or
/// table_name!(foreach(path_to::other_macro));
/// ```
/// 
/// Table rows can be matched using syntax like so when not using `foreach` mode:
/// ```rust, ignore
/// $( { $($token:tt)* } )*
/// ```
/// With `foreach` mode, you can use the pattern of the row input itself.
#[proc_macro]
pub fn table(input: TokenStream) -> TokenStream {
    let table_input = parse_macro_input!(input as TableInput);
    quote!( #table_input ).into()
}