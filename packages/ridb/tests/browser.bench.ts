
import { StorageType } from '../src/index';
import { runBenchTests, TestPlatform } from '../src/testing';

runBenchTests([TestPlatform.BROWSER], [
    {name: "InMemory", storage: StorageType.InMemory}, 
    {name: "IndexDB", storage: StorageType.IndexDB},
])
