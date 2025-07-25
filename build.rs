fn main() {
    // glib_build_tools::compile_resources(&["data"], "data/gresource.xml", "trombone.gresource");
    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        // Optional app ID
        Some("xyz.karpador.Trombone"),
        // Custom base resource path:
        // * defaults to `/com/example/myapp` in this case if not specified explicitly
        // * or `/org/relm4` if app ID was not specified either
        Some("/xyz/karpador/Trombone"),
        // Directory with custom icons (if any)
        Some("data/icons/scalable/actions"),
        // List of icons to include
        [
            "arrows-loop-tall-disabled",
            "background-app-ghost",
            "bell-outline",
            "birthday",
            "bookmark-filled",
            "bookmark-outline",
            "brain-augemnted",
            "build-alt",
            "camera-focus",
            "chat",
            "chat-bubbles-empty",
            "check-plain",
            "check-round-outline2",
            "clipboard",
            "clock-alt",
            "code",
            "cross-large",
            "dock-left",
            "dock-right",
            "earth",
            "explore2",
            "minus-circle-filled",
            "eye-not-looking",
            "eye-open-negative-filled",
            "fish",
            "funnel",
            "globe",
            "globe-small",
            "heart-broken",
            "heart-filled",
            "important-small",
            "left-large",
            "lightbulb",
            "list-compact",
            "loupe-large",
            "mail-closed-small",
            "mail-small",
            "mail-unread",
            "markdown",
            "playlist-repeat",
            "minus-large",
            "music-note",
            "network-server",
            "newspaper",
            "online",
            "padlock2-open",
            "padlock2",
            "pencil-and-paper-small",
            "paper",
            "people",
            "pin-small",
            "plus-large",
            "police-badge2",
            "quotation",
            "rich-text",
            "right-large",
            "sad-computer",
            "sentiment-dissatisfied",
            "settings",
            "smile",
            "star-filled",
            "star-outline-thick",
            "text-justify-left",
            "verified-checkmark",
            "arrow-circular-top-right",
        ],
        Some("trombone-"),
    );
}
