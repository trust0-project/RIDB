import { describe, afterAll } from 'vitest';

import { runTestsNodeOnly } from '@trust0/ridb/testing';
import {createLevelDB} from '../src';


let LevelDB = await createLevelDB();
describe('LevelDB', async () => {
    
    afterAll(async () => {
        const fs = await import('fs');
        const path = await import('path');
        fs.rmSync(path.resolve(process.cwd(), `./.db`), { recursive: true, force: true });
    });

    runTestsNodeOnly(
        [{name: "LevelDB", storage: LevelDB}]
    );

});
