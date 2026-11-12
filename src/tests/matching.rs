use crate::{config::Config, pretty_window, window_manager::Window};

const CONFIG: &str = r#"
'kitty' = 'terminal'
'brave' = 'browser'
'nvim' = 'editor'
'python' = 'python'
'youtube' = 'unused-title-rule'

[[overrides]]
app = 'BRAVE'
title = 'YouTube Music'
icon = 'music'

[[overrides]]
app = 'brave'
title = 'YouTube'
icon = 'video'

[[overrides]]
app = 'brave'
title = 'Twitch'
icon = 'stream'
"#;

fn window(class: &str, title: &str, program: Option<&str>) -> Window {
    Window {
        name: Some(title.into()),
        program: program.map(str::to_owned),
        app_id: None,
        window_properties_class: Some(class.into()),
    }
}

#[test]
fn foreground_program_precedes_terminal_regardless_of_mapping_order() {
    let config = Config::from_str(CONFIG).unwrap();
    for (program, expected) in [
        ("nvim", "editor"),
        ("python", "python"),
        ("zsh", "terminal"),
    ] {
        assert_eq!(
            pretty_window(&config, &window("kitty", "~/youtube/python", Some(program))),
            expected
        );
    }
    assert_eq!(
        pretty_window(&config, &window("kitty", "~/nvim", None)),
        "terminal"
    );
}

#[test]
fn browser_content_only_changes_icons_for_explicit_exceptions() {
    let config = Config::from_str(CONFIG).unwrap();
    for (title, expected) in [
        ("python nvim - Brave", "browser"),
        ("Video - YOUTUBE - Brave", "video"),
        ("Twitch - Brave", "stream"),
        ("YouTube Music - Brave", "music"),
    ] {
        assert_eq!(
            pretty_window(&config, &window("Brave-browser", title, None)),
            expected
        );
    }
    assert_eq!(
        pretty_window(&config, &window("kitty", "YouTube", Some("nvim"))),
        "editor"
    );
    assert_eq!(
        pretty_window(&config, &window("unknown-app", "YouTube", None)),
        "-"
    );
}

#[test]
fn wayland_app_ids_and_program_scoped_exceptions_work() {
    let mut config = Config::from_str(CONFIG).unwrap();
    let mut browser = window("", "YouTube", None);
    browser.window_properties_class = None;
    browser.app_id = Some("Brave-browser".into());
    assert_eq!(pretty_window(&config, &browser), "video");

    config.overrides[0].app = "nvim".into();
    config.overrides[0].title = "special".into();
    assert_eq!(
        pretty_window(&config, &window("kitty", "special file", Some("nvim"))),
        "music"
    );
}

#[test]
fn malformed_exceptions_fail_instead_of_matching_every_window() {
    for rule in [
        "app = ''\ntitle = 'YouTube'\nicon = 'x'",
        "app = 'brave'\ntitle = ' '\nicon = 'x'",
        "app = 'brave'\nicon = 'x'",
        "app = 'brave'\ntitle = 'YouTube'\nicon = 'x'\ntyop = 'oops'",
    ] {
        assert!(Config::from_str(&format!("[[overrides]]\n{rule}")).is_err());
    }
}

#[test]
fn separator_cannot_be_part_of_an_override_icon() {
    let config = Config::from_str(
        "[other]\nseparator = 'x'\n[[overrides]]\napp = 'brave'\ntitle = 'youtube'\nicon = 'x'",
    )
    .unwrap();
    assert_eq!(config.separator(), ": ");
}

#[test]
fn default_configuration_is_valid() {
    Config::from_str(include_str!("../../default_config.toml")).unwrap();
}
