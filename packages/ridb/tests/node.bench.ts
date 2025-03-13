
import { StorageType } from '@trust0/ridb';
import { runBenchTests, TestPlatform } from '@trust0/ridb/testing';

runBenchTests(
    [TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
