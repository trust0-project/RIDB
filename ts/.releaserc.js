// At the top of your release.config.js
const setRcPrerelease = {
    prepare: async (pluginConfig, context) => {
        if (process.env.RC === 'true') {
            context.nextRelease.version += '-rc.' + context.nextRelease.gitTag.split('.').pop();
            context.nextRelease.channel = 'rc';
            context.nextRelease.type = 'prerelease';
        }
    },
};

module.exports = {
    repositoryUrl: 'https://github.com/atala-community-projects/RIDB.git',
    branches: ['main', 'develop', { name: 'release/*' }],
    plugins: [
        '@semantic-release/commit-analyzer',
        '@semantic-release/release-notes-generator',
        setRcPrerelease, // Add the custom plugin here
        '@semantic-release/github',
        '@semantic-release/changelog',
        [
            '@semantic-release/git',
            {
                assets: ['CHANGELOG.md', 'package.json', 'package-lock.json'],
                message:
                    'chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}',
            },
        ],
        ['@semantic-release/exec', { "prepareCmd": './publish.sh ${nextRelease.version}' }],

    ],
};
