import os
from PIL import Image

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

if __name__ == "__main__":
    generate_icons()
