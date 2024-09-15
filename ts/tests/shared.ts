
import { default as Schemas } from './schemas.test';

const Tests = {
    Schemas
}
const suites = Object.values(Tests);

export enum TestPlatform {
    BROWSER = "BROWSER",
    NODE = "NODE"
}

export async function  runTests(platforms:TestPlatform[]): Promise<void> {
    platforms.forEach(platform => {
        suites.forEach(suite => {
            suite(platform)
        })
    })
}

export default {
    TestPlatform,
    runTests
}