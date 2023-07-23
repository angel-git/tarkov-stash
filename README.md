# Tarkov stash

Simple stash editor for SPT, it allows to add items in **empty** spaces of your stash

## Current supported features

### View your stash

- Stash size
- Only supported items will have icon

### Adding items to your stash

#### Currency

- Roubles

#### Meds

- Cheese

## Known issues

- You can't rotate items
- The stash size that you see might not be complete, if you need more space move items to the bottom on the client amd relaunch the app

## Features not planned

- Deleting items
- Moving items
- Guns builds

## Dev notes

All items can be fouund on this endpoint: https://dev.sp-tarkov.com/SPT-AKI/Server/raw/branch/master/project/assets/database/templates/items.json



### Dev mode

```
pnpm tauri dev
```

### production build

```
pnpm tauri build
```

# Old readme:

---

# Qwik City App ⚡️

- [Qwik Docs](https://qwik.builder.io/)
- [Discord](https://qwik.builder.io/chat)
- [Qwik GitHub](https://github.com/BuilderIO/qwik)
- [@QwikDev](https://twitter.com/QwikDev)
- [Vite](https://vitejs.dev/)

---

## Project Structure

This project is using Qwik with [QwikCity](https://qwik.builder.io/qwikcity/overview/). QwikCity is just an extra set of
tools on top of Qwik to make it easier to build a full site, including directory-based routing, layouts, and more.

Inside your project, you'll see the following directory structure:

```
├── public/
│   └── ...
└── src/
    ├── components/
    │   └── ...
    └── routes/
        └── ...
```

- `src/routes`: Provides the directory-based routing, which can include a hierarchy of `layout.tsx` layout files, and
  an `index.tsx` file as the page. Additionally, `index.ts` files are endpoints. Please see
  the [routing docs](https://qwik.builder.io/qwikcity/routing/overview/) for more info.

- `src/components`: Recommended directory for components.

- `public`: Any static assets, like images, can be placed in the public directory. Please see
  the [Vite public directory](https://vitejs.dev/guide/assets.html#the-public-directory) for more info.

## Add Integrations and deployment

Use the `pnpm qwik add` command to add additional integrations. Some examples of integrations includes: Cloudflare,
Netlify or Express Server, and
the [Static Site Generator (SSG)](https://qwik.builder.io/qwikcity/guides/static-site-generation/).

```shell
pnpm qwik add # or `yarn qwik add`
```

## Development

Development mode uses [Vite's development server](https://vitejs.dev/). The `dev` command will server-side render (SSR)
the output during development.

```shell
npm start # or `yarn start`
```

> Note: during dev mode, Vite may request a significant number of `.js` files. This does not represent a Qwik production
> build.

## Preview

The preview command will create a production build of the client modules, a production build of `src/entry.preview.tsx`,
and run a local server. The preview server is only for convenience to preview a production build locally and should not
be used as a production server.

```shell
pnpm preview # or `yarn preview`
```

## Production

The production build will generate client and server modules by running both client and server build commands. The build
command will use Typescript to run a type check on the source code.

```shell
pnpm build # or `yarn build`
```

## Static Site Generator (Node.js)

```
pnpm build.server
```
