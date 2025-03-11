import Config, {Branches, plugins} from '../../release.base.js'
export default {
  ...Config,
  tagFormat: '@trust0/ridb@${version}',
    "branches": Branches,
    "plugins": [
      [
        "@semantic-release/exec",
        {
          "prepareCmd": "sh ../../update.sh"
        }
      ],
      ...plugins
    ],
  }