
import { StorageType } from '..';

import { default as Schemas } from './test/schemas.test';
import { default as Performance } from './test/performance.test';

const Tests = {
    Schemas
}
const PerformanceTests = {
    Performance
}
const suites = Object.values(Tests);
const performanceSuites = Object.values(PerformanceTests);

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

export async function runPerformanceTests(platforms:TestPlatform[], storages:StoragesType[] ): Promise<void> {
    platforms.forEach(platform => {
        performanceSuites.forEach(suite => {
            suite(platform, storages)
        })
    })
}

export default {
    TestPlatform,
    runTests,
    runPerformanceTests
}