use kf_types::ki18n;
use qtbridge::QApp;

fn setup_localization(app: &mut QApp) {
    if let Some(engine) = app.engine.as_mut() {
        ki18n::setup_localized_context(engine);
    }
}

fn main() {
    let mut app = QApp::new();
    setup_localization(&mut app);
    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
