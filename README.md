# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

#### Credits
https://www.reddit.com/r/WutheringWaves/comments/1jm9c00/unlock_120_fps_in_version_22_new_db_method/
https://www.reddit.com/media?url=https%3A%2F%2Fpreview.redd.it%2Funlock-120-fps-in-version-2-2-new-db-method-v0-zbuk6ude1jre1.png%3Fwidth%3D788%26format%3Dpng%26auto%3Dwebp%26s%3D310e92c81c5460406c941120e62723860c73d9dc
https://github.com/WakuWakuPadoru/WuWa_Simple_FPSUnlocker
https://github.com/wakeupaj/wuwafpsunlocker