use crate::crates::arch::arch_files as arch;

mod comments;
mod constant;
mod crates;
mod data_types;
mod functions;
mod modules;
mod operators;
mod print_values;
mod variables;
fn main() {
    comments::commentsMain();
    print_values::print_valuesFn();
    variables::variableFn();
    constant::constantsFn();
    operators::operationFn();
    functions::functionFn();
    modules::playNmae("Orfeas");
    modules::playingAUdio("yolo");
    // arch_files("orfeas");
    arch("iliana");
    crates::crateFn();
    data_types::data_typesFn();
}
