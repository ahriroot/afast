// prevents TS errors

class Storage {
    key: number = 0
    constructor() {
        const timer = setInterval(() => {
            this.key++
            console.log('tick')
        }, 1000)
        console.log(timer)
    }
}

interface WorkerContext extends Worker {
    storage: Storage
}

declare var self: WorkerContext

self.storage = new Storage()

console.log(self.storage)

self.onmessage = (event: MessageEvent) => {
    const e = JSON.parse(event.data)
    console.log(e)
    postMessage(self.storage.key)
}
