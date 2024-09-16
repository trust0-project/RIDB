

module.exports = {
    repositoryUrl: 'https://github.com/atala-community-projects/RIDB.git',
    branches: [
        { name: 'rc/*',  prerelease: 'rc', channel: 'rc' },
        { name: "main" },
        'v+([0-9])?(.{+([0-9]),x}).x',
    ],
    plugins: [
        '@semantic-release/commit-analyzer',
        '@semantic-release/release-notes-generator',
        '@semantic-release/github',
        '@semantic-release/changelog',
        '@semantic-release/npm',
        [
            '@semantic-release/git',
            {
                assets: ['CHANGELOG.md', 'package.json', 'package-lock.json'],
                message:
                    'chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}',
            },
        ],

    ],
};
