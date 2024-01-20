import { App, Config } from 'afast'
import { newStorage, Storage } from 'afast/plugins/storage'

const app = new App()

const storage = newStorage()

const global: { storage: Storage } = {
    storage,
}

type Global = typeof global

app.get('/set/:key/:value', async (request, global: Global) => {
    global.storage.set(request.params.key, request.params.value, 3)
    return {}
})

app.get('/get', async (request, global: Global) => {
    return {
        storage: await global.storage.all(),
    }
})

app.get('/get/:key', async (request, global: Global) => {
    return {
        [request.params.key]: await global.storage.get(request.params.key),
    }
})

const config: Config = {
    global: global,
}

const server = app.run(config)

console.log(`Listening on ${server.url}`)
