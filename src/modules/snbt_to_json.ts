export const cleanSNBT = (snbt: string): string => {
    const combinedRegex = /\[([A-Za-z]+)\s*;\s*|([+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?)([bslfd])\b/gi;

    return snbt.replace(combinedRegex, (match, arrayType, numberVal, suffix) => {
        if (arrayType) return '[';

        if (numberVal && suffix) return numberVal;

        return match;
    });
};

export const cleanLargeSNBT = (snbt: string, chunkSize = 500000): string => {
    const chunks: string[] = [];

    for (let i = 0; i < snbt.length; i += chunkSize) {
        const chunk = snbt.slice(i, i + chunkSize);
        chunks.push(cleanSNBT(chunk));
    }

    return chunks.join('').replace(/(\d+)([bslfd])\b/gi, '$1');
};