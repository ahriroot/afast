import { App, Config } from 'afast'
import { newBidirectional, Bidirectional } from 'afast/plugins/bidirectional'

const app = new App()

const storage = newBidirectional()

const global: { storage: Bidirectional } = {
    storage,
}

type Global = typeof global

app.get('/set/:key/:value', async (request, global: Global) => {
    global.storage.setKV(request.params.key, request.params.value)
    return {}
})

app.get('/key', async (request, global: Global) => {
    return {
        storage: await global.storage.allKey(),
    }
})

app.get('/data', async (request, global: Global) => {
    return {
        storage: await global.storage.allData(),
    }
})

app.get('/key/:value', async (request, global: Global) => {
    return {
        [request.params.value]: await global.storage.getKey(request.params.value),
    }
})

app.get('/value/:key', async (request, global: Global) => {
    return {
        [request.params.key]: await global.storage.getData(request.params.key),
    }
})

const config: Config = {
    global: global,
}

const server = app.run(config)

console.log(`Listening on ${server.url}`)
