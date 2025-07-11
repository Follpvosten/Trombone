use relm4::{gtk::prelude::*, prelude::*};

use super::Place;

#[derive(Debug, Clone)]
pub struct Item {
    pub separated: bool,
    pub badge: usize,
    pub place: Place,
}

#[relm4::factory(pub)]
impl FactoryComponent for Item {
    type Init = Self;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        #[name(root)]
        gtk::ListBoxRow {
            set_activatable: true,
            set_css_classes: &["sidebar-item"],
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                #[name = "icon"]
                gtk::Image {
                    set_icon_name: Some(self.place.icon()),
                },
                #[name = "label"]
                gtk::Label {
                    set_label: &self.place.title(),
                    set_xalign: 0.,
                    set_hexpand: true,
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                },
                #[name = "badge"]
                gtk::Label {
                    set_label: &self.badge.to_string(),
                    set_visible: self.badge > 0,
                    set_valign: gtk::Align::Center,
                    set_css_classes: &["notification-badge"],
                }
            }
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets.root.set_header(None::<&gtk::Separator>);
        if self.separated {
            widgets.root.set_header(Some(
                &gtk::Separator::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .css_classes(["ttl-separator"])
                    .build(),
            ));
        }
        widgets
    }
}
