
import { StorageType } from '../src/index';
import { runTests, TestPlatform } from '../src/testing';

runTests([TestPlatform.BROWSER], [
    {name: "InMemory", storage: StorageType.InMemory}, 
    {name: "IndexDB", storage: StorageType.IndexDB}
])
