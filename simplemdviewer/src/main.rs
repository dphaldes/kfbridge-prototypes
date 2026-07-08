use qtbridge::{QApp, QmlRegister, qobject};

#[derive(Default)]
pub struct Converter {
    text: String,
}

#[qobject(NoQmlElement)]
impl Converter {
    qproperty!(
        "text",
        Member = text,
        Write = set_text,
        Notify = text_changed
    );

    fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed();
    }

    #[qsignal]
    fn text_changed(&mut self);

    #[qslot]
    fn md_format(&mut self, text: String) {
        self.set_text(markdown::to_html(text.as_str()));
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
