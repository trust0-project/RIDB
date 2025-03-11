import  {branches, plugins} from '../../release.base.js'
export default {
  tagFormat: '@trust0/ridb-level@${version}',
  branches: branches,
  plugins: [
    [
      "@semantic-release/exec",
      {
        "prepareCmd": "sh ../../update.sh"
      }
    ],
    ...plugins,
  ],
}