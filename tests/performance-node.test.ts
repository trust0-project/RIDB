import Shared from './shared';

import { StorageType } from '..';

Shared.runPerformanceTests(
    [Shared.TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
