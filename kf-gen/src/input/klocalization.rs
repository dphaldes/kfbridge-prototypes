use qtbridge_type_lib::QQmlApplicationEngine;
use std::pin::Pin;

#[qt_gen::bridge]
mod klocalization {
    include_in_cpp!(<KLocalizedQmlContext>);

    pub fn setupLocalizedContext(test: i32) {
        cpp_fn!(|test: i32| {
            todo!();
        })(test);
    }
}
