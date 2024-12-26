const fs = require('fs');
const path = require('path');

fs.writeFileSync(path.join(__dirname, '../docs/README.md'), `<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@latest/docs/logo.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>

<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    &nbsp;
    <a href="#"><img src="https://img.shields.io/npm/types/@trust0/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

# Documentation

## Modules

- [ridb](./ridb/src/README.md)
- [ridb-level](./ridb-level/src/README.md)`);



fs.writeFileSync(path.join(__dirname, '../docs/logo.svg'), `<svg 
    baseProfile="tiny" 
    version="1.2"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink" 
    viewBox="0 0 200 100"
    class="font-nunito"
>
    <!-- Add dark background rectangle -->
    <rect
        x="10"
        y="20"
        width="180"
        height="75"
        rx="15"
        ry="15"
        fill="#1a1a1a"
    />

    <text 
        x="30" 
        y="60"
        fill="#FFFFFF" 
        font-family="Nunito"
        font-size="40"
        font-weight="900" 
        letter-spacing="1" 
        style="font-stretch:120%;"
    >
        Trust
    </text>
    <text
        x="130"
        y="63"
        fill="#FF6347"
        font-family="Nunito"
        font-size="40"
        font-weight="900"
        letter-spacing="1"
        style="font-stretch:120%;"
    >
        O
    </text>

    <text
        x="58"
        y="85"
        fill="#FFF"
        font-family="Nunito"
        font-size="25"
        font-weight="900"
        letter-spacing="1"
        style="font-stretch:120%;"
    >
        RIDB
    </text>
</svg>`);