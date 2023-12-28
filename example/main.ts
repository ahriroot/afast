import { App } from 'afast'

const app = new App()

app.get('/', async (request) => {
    return {
        hello: 'world',
    }
})

const server = app.run()

console.log(`Listening on ${server.url}`)
