Workstyle
===

Sway/i3/Hyprland workspaces with style:

This application will dynamically rename your workspaces to indicate which programs are running in each one.

A picture is better than a thousand words!

* The workspace bar could look like this (uses waybar)
![alt tag](https://github.com/pierrechevalier83/workstyle/blob/master/screenshots/bar.png)

* In context:
![alt tag](https://github.com/pierrechevalier83/workstyle/blob/master/screenshots/full.png)

Note: if you are using waybar and want the workspaces to be displayed with their number, like in the screenshot, please set `"format": "{icon}",`. See [the waybar wiki](https://github.com/Alexays/Waybar/wiki/Module:-Workspaces) for more information on configuring waybar.

Installation
===

Install the latest published version from crates.io with
```
cargo install workstyle
```

Or for Arch Linux users, install the freshest git version with your favourite AUR helper, e.g.
```
yay -S workstyle-git
```

Usage
===

Simply run the executable:
```
workstyle
```

```
workspace --help
```
will give you some more context.

Sway configuration
===

Add this line to your sway config:
```
exec_always --no-startup-id workstyle &> /tmp/workstyle.log
```

You may also want to control the log level with the environment variable: RUST_LOG to error, info or debug.

Note that since your workspaces will be renamed all the time, you should configure your keybindings to use numbered workspaces instead of assuming that the name is the number:
Prefer
```
    bindsym $mod+1 workspace number 1
```
over
```
    bindsym $mod+1 workspace 1
```

Hyprland configuration
===

Add this line to your Hyprland config:
```
exec-once = workstyle &> /tmp/workstyle.log
```

SystemD integration
====

Alternatively you can use the workstyle.service file to configure systemd to automatically start workstyle after you login

Copy `workstyle.service` to `$HOME/.config/systemd/user/`

and run

```
systemctl --user enable workstyle.service
systemctl --user start workstyle.service
```

Configuration
===

The main configuration consists of deciding which icons to use for which applications.

The config file is located at `${XDG_CONFIG_HOME}/workstyle/config.toml` or `/etc/xdg/workstyle/config.toml` (the former takes precedence over the latter). It will be generated if missing. Read the generated file. The syntax is in TOML and should be pretty self-explanatory.

Ordinary mappings match application identity, never window titles or content.
GUI applications use their `app_id` or `WM_CLASS`: a Brave window stays Brave
even when its page title mentions Python, Neovim, or GitHub. Existing flat
title-based mappings must be converted to explicit exceptions to keep matching
titles.

On Linux with i3, kitty additionally uses the foreground program detected through
`/proc` and kitty's `WINDOWID` environment variable. Neovim inside kitty gets
the Neovim icon; a shell in any directory gets the kitty icon unless you explicitly
map the shell itself. Custom kitty classes such as `StartupTerminal` are supported
without changing their actual X11 class. If no program rule matches, workstyle
tries the terminal's `kitty` mapping. Foreground detection is currently specific
to kitty on X11/i3; other terminals and Wayland use their application identity.

```toml
'kitty' = ''
'nvim' = ''
'brave-browser' = ''

[[overrides]]
app = 'brave-browser'
title = 'YouTube'
icon = ''

[[overrides]]
app = 'brave-browser'
title = 'Twitch'
icon = ''

[other]
fallback_icon = '-'
```

Matching follows this order:

1. The first matching `[[overrides]]` exception, requiring **both** `app` and
   `title` to match. `app` matches the foreground program or the host application;
   `title` matches the original window title. Both selectors must be nonempty.
2. The first ordinary mapping matching the foreground program.
3. The first ordinary mapping matching the window's `app_id` or `WM_CLASS`
   (normalized to `kitty` for detected kitty windows).
4. `[other].fallback_icon` (default `-`).

All patterns are case-insensitive literal substrings, not regular expressions.
Order within each group follows the configuration file. Place specific
exceptions such as `YouTube Music` before broader ones such as `YouTube`.
Program rules beat terminal rules even if `kitty` is listed before `nvim`.
To add an exception, append another `[[overrides]]` block; remove it to restore
the application icon. An exception only inspects the title, not the page URL,
so any title containing the chosen text qualifies within the selected app.

Keep ordinary mappings at the top, before any TOML table headers. Each
`[[overrides]]` starts a new exception; `[other]` holds the general settings.
On i3, edits are picked up automatically within about one second.

The foreground job leader supplies the program name; helper commands are
ignored. Native `codex` and `ollama` processes also take precedence over their
launcher wrappers, so changing conversation titles do not affect their icons.
Background, suspended, and daemon processes are ignored. Workstyle checks at
least once per second to notice programs starting or exiting even when the
title does not change.

When an app isn't recogised in the config, `workstyle` will log the application name as an error.
Simply add that string (case insensitive) to your config file, with an icon of your choice.

If no matching icon can be found in the config, `-` will be used.
To override this, set the default icon in the config as per below:
```toml
[other]
fallback_icon = "your icon"
```

If you prefer not to have multiple copies of the same icon when there are multiple matching windows, set this config option:
```toml
[other]
deduplicate_icons = true
```

Note that the crate [`find_unicode`](https://github.com/pierrechevalier83/find_unicode/) can help find a unicode character directly from the command line. It now supports all of nerdfonts unicode space.

Minimal waybar configuration so the workspace names are showed
===

* For i3/sway
```
    "modules-left": ["sway/workspaces"],
     "sway/workspaces": {
         "format": "{icon}",
     },
```

* For hyprland
```
    "modules-left": ["wlr/workspaces"],
     "wlr/workspaces": {
         "format": "{icon}",
     },
```
