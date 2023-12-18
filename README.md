# Tauri Sveltekit Template

This repository is to be used as a template (mostly for me) for using Tauri with sveltekit and my normal stack that I'm used to. Buuuuut I will include usage instructions anyway because there are actually a few setup steps.

## Usage

1. First as always, run `yarn` or `npm i` to install all dependencies in package.json. This will include `@tauri-apps/cli` and `@tauri-apps/api` which are necesarry.
2. Run `yarn tauri init` or whatever equivalent it is for npm. You can probably find it on their website [here](https://tauri.app/v1/guides/getting-started/setup/sveltekit) and I'm to lazy to look it up rn.
3. You have to make sure of a couple of things, again its all on their [website](https://tauri.app/v1/guides/getting-started/setup/sveltekit) and its all the same steps. The main things are to make sure when it asks for build location to change it from `../public` to `../build` and when it asks about dev server to change the port from `8080` to `5173` (the default for SvelteKit, you'd have to change it in both files otherwise.), then lastly it asks about the dev server and build commands which are `vite dev` and `vite build` respectively.
