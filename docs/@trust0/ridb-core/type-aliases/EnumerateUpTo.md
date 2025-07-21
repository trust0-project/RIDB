[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / EnumerateUpTo

# Type Alias: EnumerateUpTo\<N, Acc\>

> **EnumerateUpTo**\<`N`, `Acc`\> = `Acc`\[`"length"`\] *extends* `N` ? `Acc`\[`number`\] : `EnumerateUpTo`\<`N`, \[`...Acc`, `Acc`\[`"length"`\]\]\>

Defined in: ridb\_core.d.ts:248

## Type Parameters

### N

`N` *extends* `number`

### Acc

`Acc` *extends* `number`[] = \[\]
