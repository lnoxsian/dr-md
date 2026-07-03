#!/usr/bin/env python3
import os
import argparse
from PIL import Image

def generate_logo():
    source_paths = ["assets/logo/dr-md-inapp.png", "assets/logo/dr-md-logo.png"]
    source_path = None
    for path in source_paths:
        if os.path.exists(path):
            source_path = path
            break

    if not source_path:
        print(f"Error: Source logo not found at any of: {', '.join(source_paths)}")
        return
        
    output_dir = "assets/logo"
    try:
        img = Image.open(source_path).convert("RGBA")
        print(f"Loaded source logo {source_path} ({img.size[0]}x{img.size[1]})")
        
        # Optimize size: resize to 256x256 since it's displayed at 128px
        # This reduces image size and RAM usage when loaded in egui
        optimized_size = (256, 256)
        img_optimized = img.resize(optimized_size, Image.Resampling.LANCZOS)
        
        # Save optimized dark mode version (white logo)
        dark_logo_path = os.path.join(output_dir, "dr-md-inapp-dark.png")
        img_optimized.save(dark_logo_path, "PNG", optimize=True)
        print(f"Generated optimized dark-mode logo: {dark_logo_path}")
        
        # Generate inverted light mode version (black logo)
        # We invert R, G, B channels while keeping the alpha channel intact
        r, g, b, a = img_optimized.split()
        
        # Invert colors (255 - channel)
        r_inverted = r.point(lambda p: 255 - p)
        g_inverted = g.point(lambda p: 255 - p)
        b_inverted = b.point(lambda p: 255 - p)
        
        img_inverted = Image.merge("RGBA", (r_inverted, g_inverted, b_inverted, a))
        
        # Save optimized light mode version
        light_logo_path = os.path.join(output_dir, "dr-md-inapp-light.png")
        img_inverted.save(light_logo_path, "PNG", optimize=True)
        print(f"Generated optimized light-mode logo: {light_logo_path}")
        
    except Exception as e:
        print(f"Error processing logo: {e}")

def generate_icons():
    source_path = "assets/icons/dr-md.png"
    output_dir = "assets/icons"
    
    if not os.path.exists(source_path):
        print(f"Error: Source image not found at {source_path}")
        return
        
    sizes = [16, 32, 48, 64, 128, 256, 512]
    
    try:
        img = Image.open(source_path)
        print(f"Loaded source image {source_path} ({img.size[0]}x{img.size[1]})")
        
        for size in sizes:
            # Resize using Lanczos resampling for high quality
            resized_img = img.resize((size, size), Image.Resampling.LANCZOS)
            out_path = os.path.join(output_dir, f"dr-md_{size}x{size}.png")
            resized_img.save(out_path, "PNG")
            print(f"Generated {out_path}")
            
        # Also generate an ICO file for Windows convenience
        ico_path = os.path.join(output_dir, "dr-md.ico")
        img.save(ico_path, format="ICO", sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
        print(f"Generated {ico_path}")
        
    except Exception as e:
        print(f"Error resizing image: {e}")

def main():
    parser = argparse.ArgumentParser(description="Generate app logo and icon assets.")
    parser.add_argument(
        "--type",
        choices=["all", "icons", "logo"],
        default="all",
        help="Asset type to generate (default: all)"
    )
    args = parser.parse_args()

    if args.type in ("all", "icons"):
        print("Generating application icons...")
        generate_icons()
        
    if args.type in ("all", "logo"):
        print("\nGenerating application logos...")
        generate_logo()

if __name__ == "__main__":
    main()
