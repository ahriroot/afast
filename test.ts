
import { ARequest, App } from '.'

const app = new App()

// app.get('/', async (request) => {
//     return {
//         hello: 'world',
//     }
// })

class Test {
    @app.dec.get('/')
    async index(request: ARequest) {
        return {
            hello: 'world',
        }
    }
}

const server = app.run()

console.log(`Listening on ${server.url}`)