
import { StorageType } from '../src/index';
import { runTests, TestPlatform } from '../src/testing';

runTests(
    [TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
