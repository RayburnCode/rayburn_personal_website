<!-- @format -->

# Personal Website

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```
npm install tailwindcss @tailwindcss/cli
```

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing:

```bash
dx serve
```

Or using npm:

```bash
npm run dev
```

The application will automatically load environment variables from your `.env` file through the build script.

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

```bash
dx bundle --platform web
```

```bash
cargo build
```

```bash
  window.__DXS_HYDRATION = "BASE64_ENCODED_DATA_HERE"; // Must exist!

```
