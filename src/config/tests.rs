use super::Config;
use ron::de::from_str;

const CONFIG: &str = "(
    timechange: Solar(9.4045, 10.5613),
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
    if !conf.alacritty.is_some() {
        panic!("Alacritty not detected")
    }
    if !conf.bat.is_some() {
        panic!("VSCode not detected")
    }
    if !conf.gsettings.is_some() {
        panic!("GSettings not detected")
    }
    if !conf.lighting.is_some() {
        panic!("VSCode not detected")
    }
    if !conf.mako.is_some() {
        panic!("Mako not detected")
    }
    if !conf.spotify.is_some() {
        panic!("Spotify not detected")
    }
    if !conf.vscode.is_some() {
        panic!("VSCode not detected")
    }
}
