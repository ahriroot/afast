export const sleep = async (ms: number | Date): Promise<void> => {
    return await Bun.sleep(ms)
}
