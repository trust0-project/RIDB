import Shared from './shared';

import { StorageType } from '..';

Shared.runTests(
    [Shared.TestPlatform.NODE], 
    [{name: "InMemory", storage: StorageType.InMemory}]
)
