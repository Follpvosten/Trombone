use std::{borrow::Cow, cell::RefCell, fmt::Display, rc::Rc};

use crate::{data::ListId, icons};
use relm4::{gtk::prelude::*, prelude::*};
use strum::IntoEnumIterator;

mod item;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Place {
    Static(StaticPlace),
    List(ListId, String),
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Static(place) => place.into(),
            Self::List(_, name) => name.as_str(),
        };
        write!(f, "{s}")
    }
}
impl Place {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Static(place) => place.icon(),
            Self::List(_, _) => StaticPlace::Lists.icon(),
        }
    }
    pub fn title(&self) -> Cow<'static, str> {
        match self {
            Place::Static(place) => Cow::Borrowed(place.title()),
            Place::List(_, name) => Cow::Owned(name.clone()),
        }
    }
}

#[derive(Debug, Clone, strum::IntoStaticStr, strum::EnumIter, PartialEq, Eq, Hash)]
pub enum StaticPlace {
    Home,
    Notifications,
    Conversations,
    Search,
    Favourites,
    Bookmarks,
    Hashtags,
    Explore,
    Local,
    Federated,
    Lists,
}

impl StaticPlace {
    fn icon(&self) -> &'static str {
        match self {
            StaticPlace::Home => "user-home-symbolic",
            StaticPlace::Notifications => icons::BELL_OUTLINE,
            StaticPlace::Conversations => icons::MAIL_UNREAD,
            StaticPlace::Search => icons::LOUPE_LARGE,
            StaticPlace::Favourites => icons::STAR_OUTLINE_THICK,
            StaticPlace::Bookmarks => icons::BOOKMARK_OUTLINE,
            StaticPlace::Hashtags => icons::HASHTAG,
            StaticPlace::Explore => icons::EXPLORE2,
            StaticPlace::Local => icons::NETWORK_SERVER,
            StaticPlace::Federated => icons::GLOBE,
            StaticPlace::Lists => icons::LIST_COMPACT,
        }
    }
    // idk if we'll actually need this, but nice and optimised I guess
    fn title(&self) -> &'static str {
        self.into()
    }
    fn wrap(self) -> Place {
        Place::Static(self)
    }
}

#[derive(Debug, Clone, Copy, strum::Display, strum::IntoStaticStr)]
#[strum(serialize_all = "title_case")]
pub enum SidebarMenuItem {
    NewPost,
    OpenProfile,
    Refresh,
    Announcements,
    FollowRequests,
    MutesAndBlocks,
    Drafts,
    ScheduledPosts,
    Preferences,
    KeyboardShortcuts,
    About,
    Quit,
}
impl SidebarMenuItem {
    const fn action(&self) -> &'static str {
        use crate::actions::AppAction as Action;
        match self {
            Self::NewPost => Action::NewPost.qualified_ident(),
            Self::OpenProfile => Action::OpenProfile.qualified_ident(),
            Self::Refresh => Action::Refresh.qualified_ident(),
            Self::Announcements => Action::Announcements.qualified_ident(),
            Self::FollowRequests => Action::FollowRequests.qualified_ident(),
            Self::MutesAndBlocks => Action::MutesAndBlocks.qualified_ident(),
            Self::Drafts => Action::Drafts.qualified_ident(),
            Self::ScheduledPosts => Action::ScheduledPosts.qualified_ident(),
            Self::Preferences => Action::Preferences.qualified_ident(),
            Self::KeyboardShortcuts => "win.show-help.overlay",
            Self::About => Action::About.qualified_ident(),
            Self::Quit => Action::Quit.qualified_ident(),
        }
    }
}

pub struct Sidebar {
    #[allow(dead_code)]
    curr_place: Place,
    places: Rc<RefCell<FactoryVecDeque<item::Item>>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SidebarOutput {
    PlaceChanged(Place),
    AddAccount,
    SwitchAccount(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Sidebar {
    type Init = StaticPlace;
    type Input = Place;
    type Output = SidebarOutput;

    view! {
        adw::Bin {
            set_width_request: 200,
            set_hexpand: false,
            set_css_classes: &["ttl-sidebar"],
            // set_label: "hi I am the sidebar"
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_start = &gtk::MenuButton {
                        set_tooltip_text: Some("Switch Account"),
                        #[wrap(Some)]
                        set_child = &gtk::Button {
                            adw::Avatar {
                                set_text: Some("Sylvie Hacker"),
                                set_show_initials: true,
                            }
                        },
                        set_css_classes: &["flat", "circular"],
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(menu_model),
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    adw::Banner {
                        set_revealed: false,
                        set_title: "2 Follow Requests",
                        set_button_label: Some("View"),
                    },
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_focusable: false,
                        gtk::Viewport {
                            #[wrap(Some)]
                            #[local_ref]
                            set_child = places_list -> gtk::ListBox {
                                set_selection_mode: gtk::SelectionMode::Single,
                                set_css_classes: &["navigation-sidebar"],
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        use SidebarMenuItem::*;
        let places = StaticPlace::iter()
            .map(StaticPlace::wrap)
            .chain(std::iter::once(Place::List(
                ListId("".into()),
                "frems".into(),
            )))
            .map(|place| item::Item {
                separated: matches!(
                    place,
                    Place::Static(StaticPlace::Explore) | Place::List(_, _)
                ),
                badge: 0,
                place,
            });
        let places = FactoryVecDeque::from_iter(places, gtk::ListBox::default());
        let model = Self {
            curr_place: Place::Static(init),
            places: Rc::new(RefCell::new(places)),
        };

        let places_borrow = model.places.borrow();
        let places_list = places_borrow.widget();
        let places_handle = Rc::clone(&model.places);
        let output_sender = sender.output_sender().to_owned();
        places_list.connect_row_activated(move |_box, row| {
            let places = places_handle.borrow();
            let place = places.get(row.index() as usize).expect("what");
            output_sender.emit(SidebarOutput::PlaceChanged(place.place.clone()));
        });
        let places_handle = Rc::clone(&model.places);
        places_list.set_header_func(move |row, prev| {
            let places = places_handle.borrow();
            let place = places.get(row.index() as usize).expect("what");
            let prev = prev.and_then(|row| places.get(row.index() as usize));
            row.set_header(None::<&gtk::Separator>);
            if place.separated
                && let Some(prev) = prev
                && !prev.separated
            {
                row.set_header(Some(
                    &gtk::Separator::builder()
                        .orientation(gtk::Orientation::Horizontal)
                        .css_classes(["ttl-separator"])
                        .build(),
                ));
            }
        });

        let menu_model = gio::Menu::new();

        let account_menu_model = menu_from_items(&[NewPost, OpenProfile, Refresh]);
        menu_model.append_section(None, &account_menu_model);

        let misc_menu_model = menu_from_items(&[
            Announcements,
            FollowRequests,
            MutesAndBlocks,
            Drafts,
            ScheduledPosts,
        ]);
        menu_model.append_section(None, &misc_menu_model);

        let misc_menu_model = menu_from_items(&[Preferences, KeyboardShortcuts, About, Quit]);
        menu_model.append_section(None, &misc_menu_model);

        menu_model.freeze();
        let menu_model = &menu_model;

        let widgets = view_output!();

        drop(places_borrow);

        ComponentParts { model, widgets }
    }
}

fn menu_from_items(items: &[SidebarMenuItem]) -> gio::Menu {
    let menu = gio::Menu::new();
    for item in items {
        menu.append_item(&gio::MenuItem::new(Some(item.into()), Some(item.action())));
    }
    menu
}
