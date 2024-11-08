import Shared from './shared';

import { RIDBTypes } from '..';

Shared.runTests(
    [Shared.TestPlatform.NODE], 
    [RIDBTypes.InMemory]
)