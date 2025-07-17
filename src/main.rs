#![doc = env!("CARGO_PKG_DESCRIPTION")]
use crate::actions::*;
use adw::prelude::NavigationPageExt;
use components::sidebar::{Sidebar, SidebarOutput, StaticPlace};
use gio::prelude::{ActionMapExt, ApplicationExt};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use relm4::{
    ComponentParts, ComponentSender, RelmApp, SimpleComponent,
    actions::{AccelsPlus, ActionGroupName},
    prelude::*,
};
use strum::IntoEnumIterator;

mod actions;
mod components;
mod data;

#[allow(dead_code)] // <- TODO remove once we use all icons
mod icons {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

struct AppModel {
    sidebar: Controller<Sidebar>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // remove once we actually handle all events lol
enum AppMsg {
    Sidebar(SidebarOutput),
    AppAction(AppAction),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = adw::ApplicationWindow {
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let sidebar = Sidebar::builder()
            .launch(StaticPlace::Home)
            .forward(sender.input_sender(), AppMsg::Sidebar);
        let model = AppModel { sidebar };

        // Insert the macro code generation here
        let widgets = view_output!();

        let app = relm4::main_adw_application();
        app.set_accelerators_for_action::<Quit>(&["<Ctrl>Q"]);

        let action_group = gio::SimpleActionGroup::new();
        for app_action in AppAction::iter() {
            let action = gio::SimpleAction::new(app_action.ident(), None);
            let sender = sender.clone();
            action.connect_activate(move |_, _| sender.input(AppMsg::AppAction(app_action)));
            action_group.add_action(&action);
        }
        widgets
            .main_window
            .insert_action_group(AppActions::NAME, Some(&action_group));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AppAction(AppAction::Quit) => relm4::main_adw_application().quit(),
            msg => {
                eprintln!("unimplemented message received: {msg:?}");
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("xyz.karpador.Trombone");

    relm4_icons::initialize_icons(icons::GRESOURCE_BYTES, icons::RESOURCE_PREFIX);
    relm4::set_global_css(include_str!("../data/style.css"));

    app.run::<AppModel>(0);
}
