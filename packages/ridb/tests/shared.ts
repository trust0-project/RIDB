
import { StorageType } from '..';

import { default as Schemas } from './test/schemas.test';

const Tests = {
    Schemas
}
const suites = Object.values(Tests);

export enum TestPlatform {
    BROWSER = "BROWSER",
    NODE = "NODE"
}

export type StoragesType = {
    storage: StorageType,
    name: string
}

export async function runTests(platforms:TestPlatform[], storages:StoragesType[] ): Promise<void> {
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