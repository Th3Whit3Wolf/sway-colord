use super::Config;
use ron::de::from_str;

const CONFIG: &str = "(
    timechange: Solar(9.4045, 10.5613),
    arbitrary_list: Some(
        ArbitraryList(
            arbitraries: [
                Arbitrary(
                    config_file: \"~/.config/tmux/tmux.conf\",
                    dark_line: \"source-file ~/.config/tmux/colors/spacedark-theme.tmux\",
                    light_line: \"source-file ~/.config/tmux/colors/spacelight-theme.tmux\",
                    post_hook: Some(\"tmux source-file ~/.config/tmux/tmux.conf\")
                )
            ]
        )
    ),
    alacritty: Some(
        Alacritty(
            dark_theme: Some(\"dark\"),
            light_theme: Some(\"light\")
        )
    ),
    bat: Some(
        Bat(
            dark_theme: Some(\"dark\"),
            light_theme: Some(\"light\")
        )
    ),
    gsettings: Some(
        GSettings(
            dark_gtk_theme: Some(\"Dark\"),
            dark_icon_theme: Some(\"Dark\"),
            dark_cursor_theme: None,
            dark_font_name: None,
            light_gtk_theme: Some(\"Light\"),
            light_icon_theme: Some(\"Light\"),
            light_cursor_theme: None,
            light_font_name: None,
        )
    ),
    lighting: Some(
        Lighting(
            monitor: Some(
                Monitor(
                    device: \"amdgpu_bl0\",
                    light_perc: 50,
                    dark_perc: 20
                )
            ),
            keyboard: Some(
                Keyboard(
                    device: \"kbd_backlight\",
                    light_perc: 0,
                    dark_perc: 34
                )
            )
        )
    ),
    mako: Some(
        Mako(
            dark_theme: Some(\"Space-Dark\"),
            light_theme: Some(\"Space-Light\")
        )
    ),
    spotify: Some(
        Spotify(
            dark_theme: Some(\"space-dark\"),
            light_theme: Some(\"space-light\")
        )
    ),
    vscode: Some(
        VSCode(
            dark_theme: Some(\"Dark\"),
            light_theme: Some(\"Light\")
        )
    )
)";

#[test]
fn test_read_config() {
    let conf: Config = from_str(CONFIG).unwrap();
    if conf.arbitrary_list.is_none() {
        panic!("Alacritty not detected")
    }
    if conf.alacritty.is_none() {
        panic!("Alacritty not detected")
    }
    if conf.bat.is_none() {
        panic!("VSCode not detected")
    }
    if conf.gsettings.is_none() {
        panic!("GSettings not detected")
    }
    if conf.lighting.is_none() {
        panic!("VSCode not detected")
    }
    if conf.mako.is_none() {
        panic!("Mako not detected")
    }
    if conf.spotify.is_none() {
        panic!("Spotify not detected")
    }
    if conf.vscode.is_none() {
        panic!("VSCode not detected")
    }
}
