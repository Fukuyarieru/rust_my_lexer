pub struct Token {
    name: String,
    value: String,
}
pub trait TokenTrait {
    fn identifiers() -> Vec<String>; // temporary, ideal would be Arc<[&'static str]>, Vec<String> needed for runtime
    fn name()-> String;
}