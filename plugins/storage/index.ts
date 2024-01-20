export type Key = string
export type Value = string | number | object
export type Data = { value: Value; expire?: number }

export class Storage {
    private _storage: { [x: Key]: Data } = {}
    constructor(autoClear: number = 0) {
        this._storage = {}
        if (autoClear >= 1) {
            setInterval(() => {
                Object.keys(this._storage).forEach((key) => {
                    const ex = this._storage[key].expire
                    if (ex && ex < Date.now()) {
                        delete this._storage[key]
                    }
                })
            }, autoClear * 1000)
        }
    }

    async set(key: Key, value: Value, expire?: number) {
        this._storage[key] = {
            value,
            expire: expire ? Date.now() + expire * 1000 : undefined,
        }
        return true
    }

    async get(key: Key, expire?: number): Promise<Value | null> {
        const cur = this._storage[key]
        if (cur?.expire && cur.expire < Date.now()) {
            delete this._storage[key]
            return null
        }
        if (expire !== undefined) {
            this._storage[key].expire = Date.now() + expire * 1000
        }
        return cur?.value === undefined ? null : cur.value
    }

    async del(key: Key) {
        delete this._storage[key]
        return true
    }

    async all() {
        return Object.keys(this._storage).reduce((acc: { [x: Key]: Value }, cur: Key) => {
            const current = this._storage[cur]
            if (current.expire && current.expire < Date.now()) {
                delete this._storage[cur]
            } else {
                acc[cur] = current?.value
            }
            return acc
        }, {})
    }
}

export const newStorage = (autoClear: number = 0): Storage => new Storage(autoClear)
