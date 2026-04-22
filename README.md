The table macro can be used to create a table of reusable macro input
```rust, ignore
table!{
    /// You can also add doc comments and attributes.
    /// (If you need to add attributes to the macro, this is the only
    #[macro_export]
    macro table_name [
        [row input goes here]
        [as many rows as you want]
        [use [] for rows always]
    ]
}
```
This will create a macro like this:
```rust, ignore
#[doc = "You can also add doc comments and attributes.\n(If you need 
#[macro_export]
macro_rules! table_name {
    () => {
        {row input goes here}
        {as many rows as you want}
        {use [] for rows always}
    };
    (foreach($___macro_callback:path)) => {
        $___macro_callback!{ row input goes here }
        $___macro_callback!{ as many rows as you want }
        $___macro_callback!{ use [] for rows always }
    };
    ($___macro_callback:path) => {
        $___macro_callback!{
            {row input goes here}
            {as many rows as you want}
            {use [] for rows always}
        }
    };
}
```
The created macro can be called like so:
```rust, ignore
table_name!();
// or
table_name!(path_to::other_macro);
// or
table_name!(foreach(path_to::other_macro));
```
/// 
//! 

Table rows can be matched using syntax like so when not using 

```rust, ignore
$( { $($token:tt)* } )*
```
With `foreach` mode, you can use the pattern of the row input itself.