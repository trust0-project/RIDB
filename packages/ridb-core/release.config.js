import { branches, plugins } from '../../release.base.js'
export default {
  tagFormat: '@trust0/ridb@${version}',
    "branches": branches,
    "plugins": [
      [
        "@semantic-release/exec",
        {
          "verifyConditionsCmd": "bash -c 'git diff --name-only $(git describe --tags --abbrev=0)..HEAD | grep -v \"package-lock.json\" | grep -v \"CHANGELOG.md\" | grep -v \"docs/\" | grep -v \"packages/ridb/\" | grep -v \"packages/ridb-level/\" | grep -v \"packages/ridb-react/\" | grep -v \"packages/ridb-core/CHANGELOG.md\" || exit 0' ",
          "prepareCmd": "sh ../../update.sh"
        }
      ],
      ...plugins,
    ],
  }