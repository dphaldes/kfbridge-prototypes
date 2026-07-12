use qtbridge_type_lib::QQmlApplicationEngine;
use std::pin::Pin;

#[qt_gen::bridge]
mod klocalization {
    include_in_cpp!(<KLocalizedQmlContext>);

    // type QQmlApplicationEngine = super::QQmlApplicationEngine;

    // pub fn setupLocalizedContext(engine: Pin<&mut QQmlApplicationEngine>) {
    //     cpp_fn!(|engine: Pin<&mut QQmlApplicationEngine>| {
    //         todo!();
    //     })(engine);
    // }

    pub fn setupLocalizedContext(engine: i32) {
        cpp_fn!(|engine: i32| {
            todo!();
        })(engine);
    }
}
