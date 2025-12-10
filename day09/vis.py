import sys
from PIL import Image, ImageDraw

def parse_input(input_text):
    """Parse the input text to get list of (x, y) coordinates."""
    lines = input_text.strip().split('\n')
    points = []
    for line in lines:
        x, y = map(int, line.split(','))
        points.append((x, y))
    return points

def get_line_points(p1, p2):
    """Get all points on a straight line between p1 and p2 (inclusive)."""
    x1, y1 = p1
    x2, y2 = p2
    points = []
    
    if x1 == x2:  # Vertical line
        start, end = min(y1, y2), max(y1, y2)
        for y in range(start, end + 1):
            points.append((x1, y))
    elif y1 == y2:  # Horizontal line
        start, end = min(x1, x2), max(x1, x2)
        for x in range(start, end + 1):
            points.append((x, y1))
    
    return points

def visualize_tiles(input_text, output_file='tile_map.png', image_width=1200, padding=50):
    """
    Create a PNG visualization of the tile map.
    Automatically scales coordinates to fit in a reasonable image size.
    
    Args:
        input_text: String with comma-separated x,y coordinates
        output_file: Output PNG filename
        image_width: Target width of the output image in pixels
        padding: Padding around the visualization in pixels
    """
    # Parse input
    red_points = parse_input(input_text)
    
    # Find bounds of the actual coordinates
    min_x = min(p[0] for p in red_points)
    max_x = max(p[0] for p in red_points)
    min_y = min(p[1] for p in red_points)
    max_y = max(p[1] for p in red_points)
    
    # Calculate scaling to fit in desired image size
    coord_width = max_x - min_x
    coord_height = max_y - min_y
    
    # Calculate scale to fit width, maintaining aspect ratio
    scale = (image_width - 2 * padding) / coord_width
    image_height = int(coord_height * scale) + 2 * padding
    
    def scale_point(p):
        """Scale and translate a point to image coordinates."""
        x, y = p
        scaled_x = (x - min_x) * scale + padding
        scaled_y = (y - min_y) * scale + padding
        return (scaled_x, scaled_y)
    
    # Create image
    image = Image.new('RGB', (image_width, image_height), 'white')
    draw = ImageDraw.Draw(image)
    
    # Collect all green tiles (edges) in original coordinates
    green_tiles = set()
    for i in range(len(red_points)):
        p1 = red_points[i]
        p2 = red_points[(i + 1) % len(red_points)]  # Wrap around
        line_points = get_line_points(p1, p2)
        green_tiles.update(line_points)
    
    # Remove red points from green tiles
    green_tiles -= set(red_points)
    
    # Fill interior using polygon fill
    polygon_points = [scale_point(p) for p in red_points]
    draw.polygon(polygon_points, fill='lightgreen', outline=None)
    
    # Draw green edge tiles (as lines since individual tiles would be tiny)
    # Draw the connecting lines
    for i in range(len(red_points)):
        p1 = scale_point(red_points[i])
        p2 = scale_point(red_points[(i + 1) % len(red_points)])
        draw.line([p1, p2], fill='green', width=3)
    
    # Draw red points
    point_radius = 5
    for p in red_points:
        x, y = scale_point(p)
        draw.ellipse([x - point_radius, y - point_radius, 
                     x + point_radius, y + point_radius],
                    fill='red', outline='darkred')
    
    # Add coordinate info text
    info_text = f"Range: X[{min_x}, {max_x}], Y[{min_y}, {max_y}]"
    draw.text((10, 10), info_text, fill='black')
    
    # Save image
    image.save(output_file)
    print(f"Visualization saved to {output_file}")
    print(f"Coordinate range: X=[{min_x}, {max_x}], Y=[{min_y}, {max_y}]")
    print(f"Image size: {image_width}x{image_height}")
    return image

# Example usage
if __name__ == "__main__":
    # Read input from stdin
    input_data = sys.stdin.read()
    
    # Generate visualization
    visualize_tiles(input_data, 'tile_map.png', image_width=1200)