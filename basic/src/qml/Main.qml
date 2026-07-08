import QtQuick
import QtQuick.Controls
import org.kde.kfbridges

ApplicationWindow {

    visible: true
    title: qsTr("Minimal QML app")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.say_hello()
    }
}
