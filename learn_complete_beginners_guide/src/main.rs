use crate::crates::arch::arch_files as arch;

mod comments;
mod conscurrency;
mod constant;
mod control_structures;
mod crates;
mod data_types;
mod dynamic_dispatch;
mod error_handling;
mod functions;
mod memory_mng;
mod modules;
mod operators;
mod print_values;
mod static_diatch;
mod traits;
mod variables;
fn main() {
    // comments::commentsMain();
    // print_values::print_valuesFn();
    // variables::variableFn();
    // constant::constantsFn();
    // operators::operationFn();
    // modules::playNmae("Orfeas");
    // modules::playingAUdio("yolo");
    // arch_files("orfeas");
    // arch("iliana");
    // crates::crateFn();
    // data_types::data_typesFn();
    // control_structures::control_structure();
    // functions::functionFn();
    // traits::traits_fn();
    // static_diatch::static_dispatch();
    // dynamic_dispatch::dynamic_dispatch();
    // memory_mng::onwnership::onwnership();/
    // memory_mng::borrowing::borrowing_fn();
    // memory_mng::lifetimes::lifetime_fn();
    // memory_mng::refernce_counted_variables::reference_counted_variables();
    // error_handling::working_with_files::working_with_files();
    // error_handling::errors::errors_fn();
    // error_handling::helper_methos::helper_methods();
    // error_handling::question_operator::question_operator();
    // conscurrency::trheads::threads_fn();
    // conscurrency::channels::channels_fn()
    conscurrency::mutex::mutex_fn();
}
