
import { StorageType } from '../src/index';
import { runBenchTests, TestPlatform } from '../src/testing';

runBenchTests(
    [TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
