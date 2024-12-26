
import { StorageType } from '@trust0/ridb';
import { runTests, TestPlatform } from '@trust0/ridb-testing';

runTests([TestPlatform.BROWSER], [
    {name: "InMemory", storage: StorageType.InMemory}, 
    {name: "IndexDB", storage: StorageType.IndexDB}
])
