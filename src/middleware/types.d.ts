import { Middleware } from '../types'

export type CORSConfig = {
    origin?: string
    methods?: string[]
    headers?: string[]
    credentials: boolean
}

/**
 * CORS middleware
 * @param config <afast.middleware.CORSConfig> config
 */
export declare function cors(config?: CORSConfig): Middleware {}
