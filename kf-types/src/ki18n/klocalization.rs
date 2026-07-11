// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0
//
use qtbridge_type_lib::QQmlApplicationEngine;
use std::pin::Pin;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("klocalization.h");

        // include!("qtbridge-type-lib/src/generated/qml/qqmlapplicationengine.h");
        type QQmlApplicationEngine = qtbridge_type_lib::QQmlApplicationEngine;
    }
    #[namespace = "rust::bridge::klocalization"]
    unsafe extern "C++" {
        # [rust_name = inline_cpp_fn_setup_localized_context]
        fn inlineCppFn_setupLocalizedContext(engine: Pin<&mut QQmlApplicationEngine>);
    }
}
#[allow(dead_code)]
pub fn setupLocalizedContext(engine: Pin<&mut QQmlApplicationEngine>) {
    ffi::inline_cpp_fn_setup_localized_context(engine);
}
