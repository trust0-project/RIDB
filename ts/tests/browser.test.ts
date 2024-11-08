import Shared from './shared';
import { StorageType } from '..';

Shared.runTests([Shared.TestPlatform.BROWSER], [{name: "InMemory", storage: StorageType.InMemory}, {name: "IndexDB", storage: StorageType.IndexDB}])
