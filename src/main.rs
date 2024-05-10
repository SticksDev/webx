mod b9;
mod custom_window;
mod parser;

use custom_window::Window;

use gtk::gdk::Cursor;
use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "org.bussin.napture";

#[derive(Clone, Debug)]
struct Tab {
    url: String,
    widget: gtk::Box,
    label_widget: gtk::Label,
    icon_widget: gtk::Image,
    // id: String,
}

fn main() -> glib::ExitCode {
    gtk::gio::resources_register_include!("icons.gresource").unwrap();

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| b9::css::load_css_into_app(include_str!("style.css")));
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let tabs: Vec<Tab> = vec![];

    let window = Window::new(app);

    let cursor_pointer = Cursor::from_name("pointer", None);

    let search = gtk::SearchEntry::builder().build();
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    let headerbar = gtk::HeaderBar::builder().build();

    let tabs_widget = gtk::Box::builder().css_name("tabs").spacing(6).build();

    let tab1 = make_tab(
        // tabs_widget.clone(),
        "New Tab",
        "file.png",
        cursor_pointer.as_ref(),
        tabs.clone(),
    );

    let current_tab = tab1.clone();

    tabs_widget.append(&tab1.widget);
    tabs_widget.append(&search);

    headerbar.set_title_widget(Some(&tabs_widget));

    window.set_titlebar(Some(&headerbar));

    let scroll = gtk::ScrolledWindow::builder().build();

    let scroll_clone = scroll.clone();

    search.connect_activate(move |query| {
        let mut tab_in_closure = current_tab.clone();

        // TODO: set tab_in_closure's URL to be the DNS'd URL
        tab_in_closure.url = query.text().to_string();

        query.set_text(&tab_in_closure.url.replace("buss://", ""));
        query.set_position(-1);
        query
            .root()
            .unwrap()
            .set_focus(None as Option<&gtk::Widget>);

        if let Ok(htmlview) = b9::html::build_ui(tab_in_closure.clone()) {
            scroll_clone.set_child(Some(&htmlview));
        } else {
            tab_in_closure.label_widget.set_label("HTML engine failed.");
        }
    });

    scroll.set_vexpand(true);

    let nav = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    nav.append(&separator);
    nav.append(&scroll);

    window.set_child(Some(&nav));
    window.present();
}

// commented code here was an attempt at implementing multiple tabs.
// it will be kept here in case I decide to implement multiple tabs again
fn make_tab(
    // tabs_widget: gtk::Box,
    label: &str,
    icon: &str,
    cursor_pointer: Option<&Cursor>,
    mut tabs: Vec<Tab>,
) -> Tab {
    // let tabid = gen_tab_id();

    let tab = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(12)
        .css_name("tab")
        // .css_classes(vec![tabid.clone()])
        .build();

    // let tabs_widgett = Rc::new(RefCell::new(tabs_widget));
    // let tabb = Rc::new(RefCell::new(tab.clone()));
    // let tabss = Rc::new(RefCell::new(tabs.clone()));

    // x.connect_clicked(move |_| {
    //     let tabs_widgett = Rc::clone(&tabs_widgett);
    //     let tabbb = Rc::clone(&tabb);
    //     let tabsss = Rc::clone(&tabss);

    //     remove_tab(tabbb, tabs_widgett, &mut tabsss.borrow_mut());
    // });

    let tabicon = gtk::Image::from_file(icon);

    let tabname = gtk::Label::builder()
        .css_name("tab-label")
        .label(label)
        .build();

    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed(|_gesture, _, _, _| {
        println!("I've been clicked! By tabname");
    });

    tabname.add_controller(gesture);

    tabname.set_cursor(cursor_pointer);

    tab.append(&tabicon);
    tab.append(&tabname);

    let res = Tab {
        url: "".to_string(),
        widget: tab,
        // id: tabid,
        label_widget: tabname,
        icon_widget: tabicon,
    };

    tabs.push(res.clone());

    res
}

// fn remove_tab(tab: Rc<RefCell<gtk::Box>>, tabs_widget: Rc<RefCell<gtk::Box>>, tabs: &mut Vec<Tab>) {
//     tabs_widget.borrow_mut().remove(&tab.borrow().clone());

//     tabs.retain(|potential_tab| tab.borrow().css_classes()[0] != potential_tab.id);
// }

// fn gen_tab_id() -> String {
//     use uuid::Uuid;

//     Uuid::new_v4().to_string()
// }
