export default {
  tagFormat: '@trust0/ridb@${version}',
    "branches": [
      {
        "name": "main"
      },
      {
        "name": "develop",
        "prerelease": "rc",
        "channel": "rc"
      },
      "v+([0-9])?(.{+([0-9]),x}).x"
    ],
    "plugins": [
      "@semantic-release/changelog",
      [
        "@semantic-release/commit-analyzer",
        {
          "preset": "conventionalcommits"
        }
      ],
      [
        "@semantic-release/release-notes-generator",
        {
          "preset": "conventionalcommits"
        }
      ],
      [
        "@semantic-release/exec",
        {
          "verifyConditionsCmd": "bash -c 'git diff --name-only $(git describe --tags --abbrev=0)..HEAD | grep -v \"package-lock.json\" | grep -v \"CHANGELOG.md\" | grep -v \"docs/\" | grep -v \"\.rs\" | grep -v \"\.js\" || exit 1'",
          "prepareCmd": "npx npm-check-updates -u @trust0/ridb -t newest && npx npm-check-updates -u @trust0/ridb-core -t newest && npm i && npm run docs"
        }
      ],
      "@semantic-release/github",
      "@semantic-release/npm",
      [
        "@semantic-release/git",
        {
          "assets": [
            "../../package-lock.json",
            "../../package.json",
            "package-lock.json",
            "package.json",
            "CHANGELOG.md",
            "docs/**/*"
          ],
          "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
        }
      ]
    ],
  }