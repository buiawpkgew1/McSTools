import json
import os
from PIL import Image


def get_image_color_data(image_path):
    """提取图片的四个特征RGB值和平均色域"""
    try:
        img = Image.open(image_path).convert('RGB')
    except FileNotFoundError:
        print(f"警告：未找到图片 {image_path}")
        return None

    width, height = img.size
    pixels = list(img.getdata())

    # 定义四个特征采样区域
    sample_points = {
        "low_rgb": (width // 4, height // 4),  # 左上区域中心
        "normal_rgb": (width // 2, height // 2),  # 图片中心
        "high_rgb": (3 * width // 4, height // 4),  # 右上区域中心
        "lowest_rgb": (width // 4, 3 * height // 4)  # 左下区域中心
    }

    # 获取采样点RGB值
    color_data = {}
    for name, pos in sample_points.items():
        x = min(max(pos[0], 0), width - 1)
        y = min(max(pos[1], 0), height - 1)
        color_data[name] = list(img.getpixel((x, y)))

    # 计算平均色域
    total_r = sum(p[0] for p in pixels)
    total_g = sum(p[1] for p in pixels)
    total_b = sum(p[2] for p in pixels)
    num_pixels = len(pixels)

    color_data["average_rgb"] = [
        total_r // num_pixels,
        total_g // num_pixels,
        total_b // num_pixels
    ]

    return color_data


def process_blocks(input_json, output_json, img_folder):
    """主处理函数"""
    with open(input_json, 'r') as f:
        blocks_data = json.load(f)

    result = {}

    for category, blocks in blocks_data.items():
        result[category] = {}
        for block in blocks:
            img_name = f"minecraft__{block}.png"
            img_path = os.path.join(img_folder, img_name)

            color_info = get_image_color_data(img_path)
            if color_info:
                result[category][block] = color_info

    with open(output_json, 'w') as f:
        json.dump(result, f, indent=2, ensure_ascii=False)

    print(f"处理完成！结果已保存至 {output_json}")


# 使用示例
if __name__ == "__main__":
    process_blocks(
        input_json="blocks.json",
        output_json="block_colors.json",
        img_folder="blocks"
    )