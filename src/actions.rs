relm4::new_action_group!(pub AppActions, "app");
// relm4::new_action_group!(pub WindowActions, "win");

macro_rules! actions_enum {
    ($group:ident, $vis:vis $enum:ident {$($action:ident = $name:expr,)*}) => {
        use relm4::actions::{ActionGroupName, ActionName};
        $(
            relm4::new_stateless_action!($vis $action, $group, $name);
        )*
        #[derive(Debug, Clone, Copy, strum::EnumIter)]
        pub enum $enum {
            $(
                $action,
            )*
        }
        impl $enum {
            pub const GROUP_IDENT: &str = $group::NAME;
            pub const fn ident(&self) -> &'static str {
                match self {
                    $(
                        $enum::$action => $action::NAME,
                    )*
                }
            }
            pub const fn qualified_ident(&self) -> &'static str {
                match self {
                    $(
                        $enum::$action => constcat::concat!($enum::GROUP_IDENT, ".", $action::NAME),
                    )*
                }
            }
        }
    };
}

actions_enum!(AppActions, pub AppAction {
    NewPost = "compose",
    OpenProfile = "open-current-account-profile",
    Refresh = "refresh",
    Announcements = "open-announcements",
    FollowRequests = "open-follow-requests",
    MutesAndBlocks = "open-mutes-blocks",
    Drafts = "open-draft-posts",
    ScheduledPosts = "open-scheduled-posts",
    Preferences = "open-preferences",
    About = "about",
    Quit = "quit",
});
