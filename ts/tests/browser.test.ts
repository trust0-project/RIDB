import Shared from './shared';
import { RIDBTypes } from '..';

Shared.runTests([Shared.TestPlatform.BROWSER], [RIDBTypes.InMemory, RIDBTypes.IndexDB])