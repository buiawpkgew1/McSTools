import {RequirementStatistic} from "./requirements.ts";

const escapeCsvField = (field: string): string => {
    // 处理字符串中的双引号 (转义为 "")
    let escaped = field.replace(/"/g, '""');

    // 处理换行符（Excel接受\n但需要整个字段用引号括起来）
    if (escaped.includes(',') || escaped.includes('\n') || escaped.includes('"') || escaped.includes('\r')) {
        escaped = `"${escaped}"`;
    }
    return escaped;
};

const exportCsv = (filename: string, headers: string[], rows: string[][]) => {
    const csvContent = [
        headers.join(','), // 表头行
        ...rows.map(row => row.join(',')) // 数据行
    ].join('\n');

    const blob = new Blob(['\uFEFF' + csvContent], { type: 'text/csv;charset=utf-8;' }); // \uFEFF 解决中文乱码
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);

    link.setAttribute('href', url);
    link.setAttribute('download', filename);
    link.style.visibility = 'hidden';

    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
};

export const exportRequirementsStatsToCsv = (
    name: string,
    stats: RequirementStatistic[],
    filename: string = `${name}_requirements.csv`
) => {
    const headers = ['ID', '中文', '数量', '占比 (%)'];

    const rows = stats.map(item => [
        escapeCsvField(item.id),
        escapeCsvField(item.zh_cn),
        item.num.toString(),
        item.percentage.toString()
    ]);

    exportCsv(filename, headers, rows);
};