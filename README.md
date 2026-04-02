<p align="center">
  <img src="./docs/imgs/bestofrs_logo.svg" alt="Best of RS" width="96" />
</p>

<h1 align="center">Best Of RS</h1>

<p align="center">
  A <strong>colorful</strong> Rust ecosystem tracker for discovering, comparing, and following open source project trends.
</p>

<p align="center">
  <a href="#about">About</a>
  <span> · </span>
  <a href="#architecture">Architecture</a>
  <span> · </span>
  <a href="#contribution">Contribution</a>
  <span> · </span>
  <a href="#license">License</a>
</p>

<p align="center">
  <b>README:</b>
  <a href="./README.md">English</a>
  <span> | </span>
  <a href="./docs/readme/readme_zh-CN.md">中文</a>
</p>

<p align="center">
  <b>Preview</b>
</p>

<table>
  <tr>
    <td colspan="3" align="center">
      <img src="./docs/imgs/bofrs_red.png" alt="Best Of RS preview - red" width="100%" />
    </td>
  </tr>
  <tr>
    <td colspan="3" align="center">
      <img src="./docs/imgs/bofrs_orange.png" alt="Best Of RS preview - orange" width="100%" />
    </td>
  </tr>
  <tr>
    <td colspan="3" align="center">
      <img src="./docs/imgs/bofrs_blue.png" alt="Best Of RS preview - blue" width="100%" />
    </td>
  </tr>
  <tr>
    <td width="33.33%" align="center">
      <img src="./docs/imgs/bofrs_yellow_sm.png" alt="Best Of RS small preview - yellow" width="100%" />
    </td>
    <td width="33.33%" align="center">
      <img src="./docs/imgs/bofrs_cyan_sm.png" alt="Best Of RS small preview - cyan" width="100%" />
    </td>
    <td width="33.33%" align="center">
      <img src="./docs/imgs/bofrs_green_sm.png" alt="Best Of RS small preview - green" width="100%" />
    </td>
  </tr>
  <tr>
    <td colspan="3" align="center">
      <img src="./docs/imgs/bofrs_purple.png" alt="Best Of RS preview - purple" width="100%" />
    </td>
  </tr>
</table>

## About

Best Of RS started from a simple goal: make Rust open source exploration less cold and more alive.

Inspired by **Best of JS** and built fully in **Rust**, this project tracks curated Rust repositories on **GitHub** and turns daily changes into readable trends.

It helps:

- *Users* discover active, rising, and stable Rust projects.
- *Maintainers* understand project momentum through continuous growth signals.

The tool focuses on project `metadata` and `community health`, then visualizes the ecosystem pulse across `daily`, `weekly`, `monthly`, and `yearly` windows.

## Architecture

Best Of RS is built around a curated project list, a daily tracking pipeline, and a trend-focused UI layer.

And the architecture blueprint looks like:
![Best Of RS](./docs/imgs/bestofrs_architecture.png)


See details: [Architecture](./docs/architecture/architecture.md)

## Contribution

Contributions are welcome, including **`project recommendations`**, **`bug reports`**, and **`documentation improvements`**.

```text
Preferred contribution topics:
- project recommendations
- bug reports
- documentation improvements
```

See details: [Contribution Guide](./docs/contribute/contribute.md)

## License

The source code of this repository is licensed under the **`MIT`** License (see [LICENSE](./LICENSE)).

However, the following are **not** licensed under **`MIT`** and are **reserved**:

- Project name: `Best Of RS` (and any confusingly similar name)
- Logos, icons, and brand marks
- Visual design assets (including but not limited to images, illustrations, and marketing materials)
- Website look-and-feel / UI artwork (where applicable)

You may not use the reserved items to imply endorsement, affiliation, or to create a confusingly similar service without prior written permission from the project owner.

If you would like to use any of the reserved items, please contact: [zhiyanzhaijie](https://github.com/zhiyanzhaijie)

### Notice

- This website uses **iA Writer** fonts from [iA-Fonts](https://github.com/iaolo/iA-Fonts), licensed under **SIL Open Font License 1.1 (`OFL-1.1`)**.
