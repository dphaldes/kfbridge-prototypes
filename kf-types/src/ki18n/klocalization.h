#ifndef _KLOCALIZATION_RUST_BRIDGE_H_
#define _KLOCALIZATION_RUST_BRIDGE_H_

#include <KLocalizedQmlContext>
#include <QtQml/QQmlApplicationEngine>
#include "rust/cxx.h"




namespace rust::bridge::klocalization {







void inlineCppFn_setupLocalizedContext(QQmlApplicationEngine &engine);

} // namespace rust::bridge::klocalization


#endif // _KLOCALIZATION_RUST_BRIDGE_H_
