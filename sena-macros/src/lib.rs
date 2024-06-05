use proc_macro::TokenStream;

/// Turns your struct into HList
///
/// ```
/// #[hlist]
/// pub struct Args {
///     pub id: Id,
///     pub name: String,
///
///     #[tail]
///     pub security: SecurityPipeline,
/// }
/// ```
/// This struct can be used in places where `HList` is used,
/// can be turned into `HCons` or `HNil` instances. `#[tail]` attribute is the attribute's tail,
/// e.g. `Args` can be converted to `HCons<Id, HCons<String, SecurityPipeline>>`.
pub fn hlist(args: TokenStream, body: TokenStream) -> TokenStream {
    todo!()
}
