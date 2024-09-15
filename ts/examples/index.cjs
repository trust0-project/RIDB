const {
    RIDB,
    SchemaFieldType
} = require('../build/cjs/index.cjs');

(async () => {
    const db =  new RIDB(
        {
            demo: {
                version: 0,
                primaryKey: 'id',
                type:SchemaFieldType.object,
                properties: {
                    id: {
                        type:SchemaFieldType.string,
                        maxLength: 60
                    }
                }
            }

        }
    )
    console.log("Starting the database")
    await db.start()
    console.log("Ok :)")
})()
