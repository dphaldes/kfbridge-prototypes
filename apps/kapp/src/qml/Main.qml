import QtQuick
import QtQuick.Controls
import org.kde.ki18n

ApplicationWindow {

    visible: true
    title: "KF-test!"

    Label {
        font.pixelSize: 20
        text: KI18n.i18n("This is translated?")
        anchors.centerIn : parent
    }
}
