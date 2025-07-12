use std::{borrow::Cow, fmt::Display};

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
            StaticPlace::Conversations => icons::CHAT_BUBBLES_EMPTY,
            StaticPlace::Search => icons::LOUPE_LARGE,
            StaticPlace::Favourites => icons::STAR_OUTLINE_THICK,
            StaticPlace::Bookmarks => icons::BOOKMARK_OUTLINE,
            StaticPlace::Hashtags => icons::TUBA_HASHTAG,
            StaticPlace::Explore => icons::EXPLORE2,
            StaticPlace::Local => "network-server-symbolic",
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

// TODO implement the menu lol
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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
}

pub struct Sidebar {
    #[allow(dead_code)]
    curr_place: Place,
    places: FactoryVecDeque<item::Item>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SidebarOutput {
    PlaceChanged(Place),
    AddAccount,
    SwitchAccount(String),
    MenuItemClicked(SidebarMenuItem),
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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let places = FactoryVecDeque::from_iter(
            StaticPlace::iter()
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
                }),
            gtk::ListBox::default(),
        );
        let model = Self {
            curr_place: Place::Static(init),
            places,
        };

        let places_list = model.places.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
