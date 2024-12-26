module.exports = {
    repositoryUrl: 'https://github.com/trust0-project/RIDB.git',
    branches: [
        { name: 'main' },
        { name: 'develop', prerelease: 'rc', channel: 'rc' },
        'v+([0-9])?(.{+([0-9]),x}).x',
    ],
    plugins: [
        '@semantic-release/commit-analyzer',
        '@semantic-release/release-notes-generator',
        '@semantic-release/npm',
        '@semantic-release/github',
        '@semantic-release/changelog',
        [
            '@semantic-release/git',
            {
                assets: ['CHANGELOG.md', 'package.json', 'package-lock.json', 'docs'],
                message:
                    'chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}',
            },
        ],
    ],
};
