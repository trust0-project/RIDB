import { describe, afterAll, beforeAll } from 'vitest';

import { runTests, TestPlatform } from '@trust0/ridb/testing';
import createLevelDB from '../src';



describe('LevelDB', async () => {
    let LevelDB;
    afterAll(async () => {
        const fs = await import('fs');
        const path = await import('path');
        fs.rmSync(path.resolve(process.cwd(), `./.db`), { recursive: true, force: true });
    });

    beforeAll(async () => {
        LevelDB ??= await createLevelDB();
    })

    runTests(
        [TestPlatform.NODE], 
        [{name: "LevelDB", storage: LevelDB}]
    );
});
