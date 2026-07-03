# Configuration

dr.md stores its configuration locally and recreates it on first run if it does not exist.

## Config Path

On Linux, the config file is stored at:

```text
~/.config/dr-md/config.toml
```

On Windows, the path is derived from the platform project directory helper.

## Settings

The current configuration schema includes:

- `theme` - base theme name.
- `theme_accent` - accent color selection.
- `font_size` - global UI and editor font size.
- `line_numbers` - line number visibility.
- `autosave` - autosave toggle.
- `tab_width` - tab width setting.
- `last_opened_folder` - last vault path opened by the user.

## Example

```toml
theme = "dark"
theme_accent = "blue"
font_size = 14.0
line_numbers = true
autosave = true
tab_width = 4
last_opened_folder = "/home/user/notes"
```

## Theme Notes

The app supports dark, light, Solarized dark, Solarized light, Gruvbox dark, and Gruvbox light themes. The accent color affects selection, hyperlinks, and the text cursor.

## Persistence Rules

- Opening a folder updates `last_opened_folder` and saves config immediately.
- Changing theme settings saves config immediately.
- If the config file is unreadable, the app falls back to defaults.
