# Wayland Color Daemon

This is a WIP daemon for wayland (currently tested on sway) to automatically change light and dark themes based on the time of day, and strived to eventually become a settings daemon.

## What is currently supported?

You change choose between settings a rigid timer (change occurs at 7Am & 7PM) or a solar timer (change occurs at sunrise and sunset)

Currently the following applications will switch between light and dark colorschemes automatically:

* Alacritty
* Vscode
* GTK (on sway)
  * GTK Theme
  * Icon Theme
  * Cursor Theme
  * Font Name
* Lighting
  * Monitor
  * Keyboard


## Goals

### Theme Switching

- [ ] Atom
- [ ] Spotify
- [ ] Set environment variable that can be used in scripts

### Settings

- [ ] Locale
- [ ] Sound


## How to get waybar theme to change automatically?

Waybar will change automatically with the gtk theme if you [make waybar follow the gtk theme](https://github.com/Alexays/Waybar/wiki/Styling#making-waybar-follow-the-gtk-theme).

## Instructions

Create file `~/.config/sway-colord/config.ron`

```ron
    timechange: Solar(52.4045, 0.5613),
    alacritty: Alacritty(
        dark_theme: Some("dark"),
        light_theme: Some("light")
    ),
    gsettings: GSettings(
        dark_gtk_theme: Some("Space-Dark"),
        dark_icon_theme: Some("Space-Dark"),
        dark_cursor_theme: None,
        dark_font_name: None,
        light_gtk_theme: Some("Space-Light"),
        light_icon_theme: Some("Space-Light"),
        light_cursor_theme: None,
        light_font_name: None,
    ),
    lighting: Lighting(
        monitor: Some(
            Monitor(
                device: "amdgpu_bl0",
                light_perc: 50,
                dark_perc: 20
            )
        ),
        keyboard: Some(
            Keyboard(
                device: "asus::kbd_backlight",
                light_perc: 0,
                dark_perc: 34
            )
        )
    ),
    vscode: VSCode(
        dark_theme: Some("Spacemacs - dark"),
        light_theme: Some("Spacemacs - light")
    )
)
```
