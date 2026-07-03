# Assets

All generated application artwork lives under `assets/`.

The project separates branding into two groups:

- `assets/logo/` for the app logo and in-app branding.
- `assets/icons/` for window and launcher icon variants.

## Logos

The logo files are stored in:

- `assets/logo/dr-md-logo.png`
- `assets/logo/dr-md-inapp-light.png`
- `assets/logo/dr-md-inapp-dark.png`

## Logo Usage

- `dr-md-logo.png` is the main brand mark.
- `dr-md-inapp-light.png` is intended for light UI surfaces.
- `dr-md-inapp-dark.png` is intended for dark UI surfaces.

These assets are generated so the in-app branding and packaging graphics stay visually consistent across releases.

## Icons

The app icons are stored in:

- `assets/icons/dr-md.ico`
- `assets/icons/dr-md.png`
- `assets/icons/dr-md_16x16.png`
- `assets/icons/dr-md_32x32.png`
- `assets/icons/dr-md_48x48.png`
- `assets/icons/dr-md_64x64.png`
- `assets/icons/dr-md_128x128.png`
- `assets/icons/dr-md_256x256.png`
- `assets/icons/dr-md_512x512.png`

## Icon Usage

The smaller sizes are useful for window decorations and desktop launchers, while the larger PNGs support packaging and future distribution formats.

The 256x256 PNG is also embedded at startup as the native window icon in `src/main.rs`.

## Regeneration

Use the asset generation script when the source artwork changes:

```bash
make generate-icons
make generate-logos
make generate-assets
```

The generator script is `scripts/generate_assets.py`.

If you change the source artwork, regenerate both logos and icons before cutting a release so the window icon, branding assets, and package assets remain in sync.
