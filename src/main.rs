#![doc = env!("CARGO_PKG_DESCRIPTION")]
use components::sidebar::{Sidebar, StaticPlace};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use relm4::{
    adw::{self, prelude::NavigationPageExt},
    gtk::{self, gdk},
    prelude::*,
    ComponentParts, ComponentSender, RelmApp, SimpleComponent,
};

mod components;
mod data;

#[allow(dead_code)] // <- TODO remove once we use all icons
mod icons {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

struct AppModel {
    sidebar: Controller<Sidebar>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = ();
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Trombone"),
            set_width_request: 360,
            set_height_request: 294,

            adw::ToastOverlay {
                gtk::Overlay {
                    adw::OverlaySplitView {
                        set_sidebar = Some(model.sidebar.widget()),
                        #[wrap(Some)]
                        #[name = "nav_view"]
                        set_content = &adw::NavigationView {
                            #[name = "main_page"]
                            add = &adw::NavigationPage {
                                set_title: "Main",
                                #[wrap(Some)]
                                set_child = &adw::ToolbarView {
                                    add_top_bar = &adw::HeaderBar {
                                        #[wrap(Some)]
                                        set_title_widget = &adw::WindowTitle {
                                            set_title: "Main",
                                        },
                                    },
                                    gtk::Label {
                                        set_label: "hello I am the main page"
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let sidebar = Sidebar::builder().launch(StaticPlace::Home).detach();
        let model = AppModel { sidebar };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn load_custom_css() {
    gio::resources_register_include!("trombone.gresource").unwrap();
    let display = gdk::Display::default().unwrap();
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/xyz/karpador/Trombone/style.css");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = RelmApp::new("xyz.karpador.Trombone");

    relm4_icons::initialize_icons(icons::GRESOURCE_BYTES, icons::RESOURCE_PREFIX);

    load_custom_css();

    app.run::<AppModel>(0);
}
