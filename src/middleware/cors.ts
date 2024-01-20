import type { ARequest, AResponse, Middleware } from '../../types'
import { CORSConfig } from './types'

class CORS implements Middleware {
    constructor(private config: CORSConfig) {
        this.config = config
    }

    async request(request: ARequest, ws?: any): Promise<ARequest | AResponse> {
        return request
    }

    async response(request: ARequest, response: AResponse): Promise<AResponse> {
        if (response instanceof Response) {
            response.headers.set('Access-Control-Allow-Origin', this.config.origin || '*')
            response.headers.set(
                'Access-Control-Allow-Methods',
                this.config.methods?.join(',') || 'GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD'
            )
            response.headers.set('Access-Control-Allow-Headers', this.config.headers?.join(',') || '')
            response.headers.set('Access-Control-Allow-Credentials', this.config.credentials ? 'true' : 'false')
            return response
        } else if (response instanceof Error) {
            return response
        } else {
            return new Response(JSON.stringify(response), {
                headers: {
                    'Access-Control-Allow-Origin': this.config.origin || '*',
                    'Access-Control-Allow-Methods':
                        this.config.methods?.join(',') || 'GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD',
                    'Access-Control-Allow-Headers': this.config.headers?.join(',') || '',
                    'Access-Control-Allow-Credentials': this.config.credentials ? 'true' : 'false',
                },
            })
        }
    }
}

export const cors = (
    config: CORSConfig = {
        origin: '*',
        methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS', 'HEAD'],
        headers: [],
        credentials: false,
    }
) => new CORS(config)
