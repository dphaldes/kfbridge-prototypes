use cmake_package;
use qtbridge_build_utils::qt_build::QtInstallation;
use std::path::Path;

pub const GENERATED_FILES_BRIDGE: [&'static str; 1] = ["src/ki18n/klocalization.rs"];
pub const GENERATED_FILES_CPP: [&'static str; 1] = ["src/ki18n/klocalization.cpp"];
const LIBRARIES: &[(&'static str, &[&'static str])] =
    &[("KF6I18n", &["KF6::I18n", "KF6::I18nQml"])];

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = std::path::Path::new(&manifest_dir).join("src");

    // This becomes DEP_QTBRIDGE_TYPE_LIB_INCLUDE in dependents
    println!("cargo:include={}", include_path.display());
    println!("cargo::metadata=include={}", include_path.display());

    let qt = QtInstallation::default();
    for file in GENERATED_FILES_BRIDGE {
        println!("cargo::rerun-if-changed={file}");
    }
    let mut builder = cxx_build::bridges(GENERATED_FILES_BRIDGE);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include("../");
    qt.configure_builder(&mut builder);

    GENERATED_FILES_CPP.iter().for_each(|file| {
        builder.file(file);
        println!("cargo::rerun-if-changed={file}");
        let h_path = Path::new(file).with_extension("").with_extension("h");
        if h_path.is_file() {
            println!("cargo::rerun-if-changed={}", h_path.to_str().unwrap());
        }
    });

    let qt_modules = ["Core", "Gui", "Qml", "QmlIntegration"];
    for include_dir in qt.include_dirs(qt_modules, true) {
        builder.include(include_dir);
    }
    let qt_modules = ["Core", "Gui", "Qml"];
    qt.link_modules(qt_modules);
    link_libraries(&mut builder);

    builder.compile("kde-frameworks");
}

fn link_libraries(builder: &mut cc::Build) {
    let mut directories = Vec::new();

    for (name, targets) in LIBRARIES {
        match cmake_package::find_package(*name).find() {
            Err(err) => panic!("Cannot find {name}: {err:?}"),
            Ok(package) => {
                for target in *targets {
                    let cmake_target = package.target(target.to_owned()).unwrap();
                    cmake_target.link();
                    for dir in cmake_target.include_directories {
                        directories.push(dir);
                    }
                }
            }
        }
    }

    for dir in &directories {
        builder.include(dir);
    }
}
