[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / ExtractObject

# Type Alias: ExtractObject\<PR, R\>

> **ExtractObject**\<`PR`, `R`\> = `{ [K in keyof PR as IsNestedOptional<PR, K, R> extends true ? never : K]: ExtractProperty<PR[K]> }` & `{ [K in keyof PR as IsNestedOptional<PR, K, R> extends true ? K : never]?: ExtractProperty<PR[K]> }`

Defined in: ridb\_core.d.ts:629

ExtractObject builds an object document type from a `properties` map `PR` and the
owning object's required-name union `R`, applying the correct optional/required
modifier to each nested property (see [IsNestedOptional](IsNestedOptional.md)). This keeps `Doc` and
`CreateDoc` in step with the runtime validator, which only enforces nested keys named
in that object's `required` array.

## Type Parameters

### PR

`PR`

### R

`R`
