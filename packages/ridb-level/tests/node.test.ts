
import { runTests, TestPlatform } from '@trust0/ridb-testing';
import { LevelDB } from '../src';
runTests(
    [TestPlatform.NODE], 
    [{name: "LevelDB", storage: LevelDB}]
)
