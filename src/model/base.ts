import type { Options } from '../../types'

export const Field = (
    options: Options = {
        primary: false,
        show: true,
        name: '',
        property: '',
    }
) => {
    return (target: any, propertyKey: any) => {
        if (options.show === undefined) {
            options.show = true
        }
        options.property = propertyKey
        options.name = options.name || propertyKey

        if (!['String', 'Number', 'Date'].includes(target.constructor.name)) {
            let models = Reflect.getMetadata('models', global) || {}
            if (!Object.keys(models).includes(target.constructor.name)) {
                models[target.constructor.name] = target
                Reflect.defineMetadata('models', models, global)
            }
        }

        if (options.type === 'Foreign' && options.foreign !== undefined) {
            const f = new options.foreign()
            options.references = f.primaryKey().name
        }

        let fields = Reflect.getMetadata('fields', target) || []
        fields.push(options)

        Reflect.defineMetadata('fields', fields, target)
    }
}
