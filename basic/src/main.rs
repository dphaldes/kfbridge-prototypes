use qtbridge::{QApp, QmlRegister, qobject};

#[derive(Default)]
pub struct Backend {}

#[qobject(NoQmlElement)]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

impl QmlRegister for Backend {
    const URI: &str = "org.kde.kfbridges";
    const ELEMENT_NAME: &str = "Backend";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = true;
}

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
