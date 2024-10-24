use gtk::prelude::IsA;

use super::bt_connection;

pub fn header() -> impl IsA<gtk::Widget> {
    let header = adw::HeaderBar::new();
    header.pack_start(&bt_connection::BtConnection::new());
    header
}
