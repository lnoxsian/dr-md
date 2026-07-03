import os
from PIL import Image

def generate_logo():
    source_path = "assets/logo/dr-md-inapp.png"
    output_dir = "assets/logo"
    
    if not os.path.exists(source_path):
        print(f"Error: Source logo not found at {source_path}")
        return
        
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

if __name__ == "__main__":
    generate_logo()
