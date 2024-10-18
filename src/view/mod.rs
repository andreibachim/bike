mod pages;
use gtk::prelude::IsA;

pub fn navigation_view() -> impl IsA<gtk::Widget> {
    let navigation_view = adw::NavigationView::new();
    navigation_view.add(&pages::routes_page());
    navigation_view
}
