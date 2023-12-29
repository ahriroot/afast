export type Key = string
export type Data = string | number
export type ValueData = { data: Data; expire?: number }
export type ValueKey = { key: Key; expire?: number }

export class Bidirectional {
    private kv: { [x: Key]: ValueData }
    private vk: { [x: Data]: ValueKey }
    constructor(autoClear: number = 0) {
        this.kv = {}
        this.vk = {}
        if (autoClear > 1) {
            setInterval(() => {
                Object.keys(this.kv).forEach((key) => {
                    const cur = this.kv[key]
                    if (cur !== undefined) {
                        if (cur.expire && cur.expire < Date.now()) {
                            delete this.kv[key]
                            delete this.vk[cur.data]
                        }
                    }
                })
            }, autoClear * 1000)
        }
    }

    async setKV(key: Key, data: Data, expire?: number) {
        const ex = expire ? Date.now() + expire * 1000 : undefined
        this.kv[key] = { data, expire: ex }
        this.vk[data] = { key, expire: ex }
        return true
    }

    async setVK(data: Data, key: Key, expire?: number) {
        return this.setKV(key, data, expire)
    }

    async getKey(data: Data, expire?: number) {
        const cur = this.vk[data]
        if (cur === undefined) {
            return undefined
        }
        if (cur.expire && cur.expire < Date.now()) {
            delete this.vk[data]
            delete this.kv[cur.key]
            return undefined
        }
        if (expire !== undefined) {
            this.vk[data].expire = Date.now() + expire * 1000
            this.kv[cur.key].expire = Date.now() + expire * 1000
        }
        return cur.key
    }

    async getData(key: Key, expire?: number) {
        const cur = this.kv[key]
        if (cur === undefined) {
            return undefined
        }
        if (cur.expire && cur.expire < Date.now()) {
            delete this.kv[key]
            delete this.vk[cur.data]
            return undefined
        }
        if (expire !== undefined) {
            this.kv[key].expire = Date.now() + expire * 1000
            this.vk[cur.data].expire = Date.now() + expire * 1000
        }
        return cur.data
    }

    async delByKey(key: Key) {
        const cur = this.kv[key]
        delete this.kv[key]
        if (cur !== undefined) {
            delete this.vk[cur.data]
        }
        return true
    }

    async delByData(data: Data) {
        const cur = this.vk[data]
        delete this.vk[data]
        if (cur !== undefined) {
            delete this.kv[cur.key]
        }
        return true
    }

    async allKey() {
        return Object.keys(this.vk).reduce((acc: { [x: Data]: Key }, cur: Data) => {
            const current = this.vk[cur]
            if (current.expire && current.expire < Date.now()) {
                delete this.vk[cur]
                if (current !== undefined) {
                    delete this.kv[current.key]
                }
            } else {
                acc[cur] = this.vk[cur].key
            }
            return acc
        }, {})
    }

    async allData() {
        return Object.keys(this.kv).reduce((acc: { [x: Key]: Data }, cur: Key) => {
            const current = this.kv[cur]
            if (current.expire && current.expire < Date.now()) {
                delete this.kv[cur]
                if (current !== undefined) {
                    delete this.vk[current.data]
                }
            } else {
                acc[cur] = this.kv[cur].data
            }
            return acc
        }, {})
    }
}

export const newBidirectional = (autoClear: number = 0): Bidirectional => new Bidirectional(autoClear)
