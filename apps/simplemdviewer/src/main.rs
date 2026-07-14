use qtbridge::{QApp, QmlRegister, qobject};

#[derive(Default)]
pub struct Converter {}

#[qobject(NoQmlElement)]
impl Converter {
    #[qslot]
    fn md_format(&mut self, text: String) -> String {
        markdown::to_html(text.as_str())
    }
}

impl QmlRegister for Converter {
    const URI: &str = "org.kde.simplemdviewer";
    const ELEMENT_NAME: &str = "MdConverter";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = false;
}

fn main() {
    QApp::new()
        .register::<Converter>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
