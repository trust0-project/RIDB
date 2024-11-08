
import { BaseStorage, SchemaType, RIDBTypes } from '..';
import { default as Schemas } from './schemas.test';

const Tests = {
    Schemas
}
const suites = Object.values(Tests);

export enum TestPlatform {
    BROWSER = "BROWSER",
    NODE = "NODE"
}

export async function  runTests(platforms:TestPlatform[], storages:(typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>)[] ): Promise<void> {
    platforms.forEach(platform => {
        suites.forEach(suite => {
            suite(platform, storages)
        })
    })
}

export default {
    TestPlatform,
    runTests
}