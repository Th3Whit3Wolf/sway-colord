# Sway Color Daemon

This is a WIP daemon for sway to automatically change light and dark themes based on the time of day.

## What is currently supported?

You change choose between settings a rigid timer (change occurs at 7Am & 7PM) or a solar timer (change occurs at sunrise and sunset)

Currently the following applications will switch between light and dark colorschemes automatically:

* Alacritty
* Bat
* Vscode
* GTK (on sway)
  * GTK Theme
  * Icon Theme
  * Cursor Theme
  * Font Name
* Lighting
  * Monitor
  * Keyboard

Sway Color Daemon also creates `/tmp/sway-colord/dawn` & `/tmp/sway-colord/dusk` files
that can be read to find out when the next timechange is from a shell script

## Goals

### Theme Switching

- [ ] Atom
- [ ] Spotify

### UI

- [ ] tui
- [ ] gui

## Instructions

Create file `~/.config/sway-colord/config.ron`

```ron
(
    timechange: Solar(52.4045, 0.5613),
    alacritty: Some
	Alacritty(
	    dark_theme: Some("dark"),
	    light_theme: Some("light")
	)
    ),
    bat: Some(
	Bat(
	    dark_theme: Some("OneHalfDark"),
	    light_theme: Some("OneHalfLight")
	)
    ),
    gsettings: Some(
	GSettings(
	    dark_gtk_theme: Some("Space-Dark"),
	    dark_icon_theme: Some("Space-Dark"),
	    dark_cursor_theme: None,
	    dark_font_name: None,
	    light_gtk_theme: Some("Space-Light"),
	    light_icon_theme: Some("Space-Light"),
	    light_cursor_theme: None,
	    light_font_name: None,
	)
    ),
    lighting: Some(
	Lighting(
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
	)
    ),
    vscode: Some(
	VSCode(
	    dark_theme: Some("Spacemacs - dark"),
	    light_theme: Some("Spacemacs - light")
	)
    )
)
```

## How to get waybar theme to change automatically?

Waybar will change automatically with the gtk theme if you [make waybar follow the gtk theme](https://github.com/Alexays/Waybar/wiki/Styling#making-waybar-follow-the-gtk-theme).

## How to get my other utilities to switch themes with sway-colord?

You can make an executable script for any utility that accepts a theme or config as a paramete. Place it in `~/.local/bin` and make sure `~/.local/bin` is in your `$PATH`. Here is an example for wofi.

```shell
#!/usr/bin/env bash

# place this in ~/.local/bin/wofi
dawn=$(cat /tmp/sway-colord/dawn)
dusk=$(cat /tmp/sway-colord/dusk)
now=$(date +%H:%M)

if [[ "$now" < "$dawn" ]] || [[ "$now" > "$dusk" ]]; then
    # Dark Theme
    /usr/bin/wofi -s ~/.config/wofi/dark.css
else
    # Light Theme
    /usr/bin/wofi -s ~/.config/wofi/light.css
fi
```

