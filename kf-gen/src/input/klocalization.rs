use crate::QQmlApplicationEngine;
use std::pin::Pin;

#[qt_gen::bridge]
mod klocalization {
    include_in_cpp!(<KLocalizedQmlContext>);

    pub fn setupLocalizedContext(engine: Pin<&mut QQmlApplicationEngine>) {
        cpp_fn!(|engine: Pin<&mut QQmlApplicationEngine>| {
            todo!();
        })();
    }
}
