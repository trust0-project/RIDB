

import { StorageClass, StorageType } from '../types';
import { UnitTests, BenchTests } from './test/schemas.test';


const suites = Object.values({
    UnitTests
});


const benchSuites = Object.values({
    BenchTests
});


export enum TestPlatform {
    BROWSER = "BROWSER",
    NODE = "NODE"
}

export type StoragesType = {
    storage: StorageType | StorageClass<any>,
    name: string
}

export async function runTests(platforms:TestPlatform[], storages:StoragesType[] ): Promise<void> {
    platforms.forEach(platform => {
        suites.forEach(suite => {
            suite(platform, storages)
        })
    })
}

export async function runTestsNodeOnly(  storages:StoragesType[] ): Promise<void> {
    suites.forEach(suite => {
        suite(TestPlatform.NODE, storages)
    })
}

export async function runBenchTests(platforms:TestPlatform[], storages:StoragesType[] ): Promise<void> {
    platforms.forEach(platform => {
        benchSuites.forEach(suite => {
            suite(platform, storages)
        })
    })
}

export async function runBenchTestsNodeOnly(  storages:StoragesType[] ): Promise<void> {
    benchSuites.forEach(suite => {
        suite(TestPlatform.NODE, storages)
    })
}

export default {
    TestPlatform,
    runTests,
    runBenchTests,
    runTestsNodeOnly,
    runBenchTestsNodeOnly
}