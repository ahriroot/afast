
import { ARequest, App } from '.'

const app = new App()

// app.get('/', async (request) => {
//     return {
//         hello: 'world',
//     }
// })

class Test {
    @app.dec.get('/:p:number/:q:string')
    async index(request: ARequest) {
        console.log(request)
        return {
            hello: 'world',
        }
    }
}

const server = app.run()

console.log(`Listening on ${server.url}`)