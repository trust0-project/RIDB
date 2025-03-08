import { describe, afterAll } from 'vitest';

import { runTests, TestPlatform } from '@trust0/ridb/testing';
import { LevelDB } from '../src';



describe('LevelDB', () => {

    afterAll(async () => {
        const fs = await import('fs');
        const path = await import('path');
        fs.rmSync(path.resolve(process.cwd(), `./.db`), { recursive: true, force: true });
    });

    runTests(
        [TestPlatform.NODE], 
        [{name: "LevelDB", storage: LevelDB}]
    );
});
