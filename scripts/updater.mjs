// 注意要安装@actions/github依赖
import { context, getOctokit } from "@actions/github";
import { readFile } from "node:fs/promises";

// 定义镜像代理前缀
const MIRROR_PREFIX = "http://ghcr.mcschematic.top:5000/";

// 在容器中可以通过env环境变量来获取参数
const octokit = getOctokit(process.env.GITHUB_TOKEN);

const updateRelease = async () => {
    // 获取updater tag的release
    const { data: release } = await octokit.rest.repos.getReleaseByTag({
        owner: context.repo.owner,
        repo: context.repo.repo,
        tag: "updater",
    });

    // 删除旧的的文件
    const deletePromises = release.assets
        .filter((item) => item.name === "latest.json")
        .map(async (item) => {
            await octokit.rest.repos.deleteReleaseAsset({
                owner: context.repo.owner,
                repo: context.repo.repo,
                asset_id: item.id,
            });
        });

    await Promise.all(deletePromises);

    // 读取并修改latest.json
    const rawContent = await readFile("latest.json", { encoding: "utf-8" });
    const jsonContent = JSON.parse(rawContent);

    // 遍历所有平台添加代理前缀
    for (const platform of Object.keys(jsonContent.platforms)) {
        const originalUrl = jsonContent.platforms[platform].url;
        // 确保URL没有重复前缀
        if (!originalUrl.startsWith(MIRROR_PREFIX)) {
            jsonContent.platforms[platform].url = `${MIRROR_PREFIX}${originalUrl}`;
        }
    }

    // 转换回字符串
    const modifiedContent = JSON.stringify(jsonContent, null, 2);

    // 上传新的文件
    await octokit.rest.repos.uploadReleaseAsset({
        owner: context.repo.owner,
        repo: context.repo.repo,
        release_id: release.id,
        name: "latest.json",
        data: modifiedContent,
    });
};

updateRelease();