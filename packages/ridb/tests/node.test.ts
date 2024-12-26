
import { StorageType } from '@trust0/ridb';
import { runTests, TestPlatform } from '@trust0/ridb-testing';

runTests(
    [TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
