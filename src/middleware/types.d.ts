import { ARequest, AResponse } from '../../types'
import { Middleware } from '../types'

export type CORSConfig = {
    origin?: string
    methods?: string[]
    headers?: string[]
    credentials?: boolean
    skip?: (request: ARequest, response: AResponse, global: any) => boolean
}

/**
 * CORS middleware
 * @param config <afast.middleware.CORSConfig> config
 */
export declare function cors(config?: CORSConfig): Middleware {}
