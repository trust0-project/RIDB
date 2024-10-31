/// <reference types="@hyperledger/identus-edge-agent-sdk" />

const {
    RIDB,
} = require('../build/cjs/index.js');

const SDK = require("@hyperledger/identus-edge-agent-sdk");

(async () => {
    const db =  new RIDB(
        {
            credential: SDK.default.Models.CredentialSchema

        }
    )
    console.log("Starting the database")
    await db.start()
    console.log("Ok :)")
})()
