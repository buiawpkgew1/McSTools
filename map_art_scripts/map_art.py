import json
import os
from PIL import Image


def rgb_to_hex(rgb_tuple):
    return "#{:02x}{:02x}{:02x}".format(*rgb_tuple)


def get_image_color_data(image_path):
    try:
        img = Image.open(image_path).convert('RGB')
    except FileNotFoundError:
        print(f"警告：未找到图片 {image_path}")
        return None

    width, height = img.size
    pixels = list(img.getdata())

    sample_points = {
        "left_top": (width // 4, height // 4),
        "center": (width // 2, height // 2),
        "right_top": (3 * width // 4, height // 4),
        "left_bottom": (width // 4, 3 * height // 4)
    }

    color_data = {}
    for name, pos in sample_points.items():
        x = min(max(pos[0], 0), width - 1)
        y = min(max(pos[1], 0), height - 1)
        color_data[name] = list(img.getpixel((x, y)))

    total_r = sum(p[0] for p in pixels)
    total_g = sum(p[1] for p in pixels)
    total_b = sum(p[2] for p in pixels)

    avg_rgb = (
        total_r // len(pixels),
        total_g // len(pixels),
        total_b // len(pixels)
    )

    color_data["average_rgb"] = list(avg_rgb)
    color_data["average_rgb_hex"] = rgb_to_hex(avg_rgb)
    return color_data


def process_blocks(input_json, output_json, img_folder, je_blocks_json):
    """主处理流程（添加中文译名支持）"""
    with open(je_blocks_json, 'r', encoding='utf-8') as f:
        je_blocks = json.load(f)

    id_to_chinese = {}
    for entry in je_blocks:
        id_parts = entry['ID'].split(',')
        last_part = id_parts[-1].strip()
        block_id = last_part.split('.')[-1]
        id_to_chinese[block_id] = entry['n']

    with open(input_json, 'r', encoding='utf-8') as f:
        blocks_data = json.load(f)

    result = {}
    for category, blocks in blocks_data.items():
        result[category] = {}
        for block in blocks:
            img_path = os.path.join(img_folder, f"{block}.png")
            color_info = get_image_color_data(img_path)
            if color_info:
                color_info['zh_cn'] = id_to_chinese.get(block, '未知')
                result[category][block] = color_info

    with open(output_json, 'w', encoding='utf-8') as f:
        json.dump(result, f, indent=2, ensure_ascii=False)
    print(f"处理完成！结果已保存至 {output_json}")


if __name__ == "__main__":
    process_blocks(
        input_json="blocks.json",
        output_json="scaled_block_colors.json",
        img_folder="blocks_img",
        je_blocks_json="je_blocks.json"
    )