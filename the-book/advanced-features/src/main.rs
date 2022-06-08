mod unsafes;
mod traits;
mod types;
mod functions;
mod macros;

fn main() {
    unsafes::unsafe_examples();
    unsafes::unsafe_functions();
    unsafes::extern_functions();
    unsafes::global_variables();

    traits::operator_overload();
    traits::disambiguation();
    traits::supertraits();
    traits::newtype();

    types::aliases();
    types::use_result().unwrap();

    functions::functions();
    functions::closures();

    macros::declarative();
    macros::procedural();
}
