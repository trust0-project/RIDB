module.exports = {
    repositoryUrl: 'https://github.com/trust0-project/ridb-wasm.git',
    branches: [
        { name: 'main' },
        { name: 'develop', prerelease: 'rc', channel: 'rc' },
        'v+([0-9])?(.{+([0-9]),x}).x',
    ],
    plugins: [
        '@semantic-release/commit-analyzer',
        '@semantic-release/release-notes-generator',
        [
            '@semantic-release/npm',
            {
                "pkgRoot": "./pkg",
                "token": "${NPM_TOKEN}"
            }
        ],
        '@semantic-release/github',
        '@semantic-release/changelog',
        [
            '@semantic-release/exec',
            {
                prepareCmd: 'node scripts/update-cargo-version.js ${nextRelease.version} && cargo check'
            }
        ],
        [
            '@semantic-release/git',
            {
                assets: ['Cargo.toml', 'Cargo.lock', 'CHANGELOG.md'],
                message:
                    'chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}',
            },
        ],
    ],
};
