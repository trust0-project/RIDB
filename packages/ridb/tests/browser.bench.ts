
import { StorageType } from '@trust0/ridb';
import { runBenchTests, TestPlatform } from '@trust0/ridb/testing';

runBenchTests([TestPlatform.BROWSER], [
    {name: "InMemory", storage: StorageType.InMemory}, 
    {name: "IndexDB", storage: StorageType.IndexDB},
])
