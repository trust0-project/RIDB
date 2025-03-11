export const branches = [
  {
    name: "main"
  },
  {
    name: "develop",
    prerelease: "rc",
    channel: "rc"
  },
  "v+([0-9])?(.{+([0-9]),x}).x"
]



export const plugins = [
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
]