module.exports = {
  content: [
    "./src/routes/**/*.{svelte,js,ts}",
    "./src/lib/**/*.{svelte,js,ts}",
  ],
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        nerds: {
          primary: "#ed1e79",
          secondary: "#ee1b24",
          accent: "#0164b0",
          neutral: "#ffffff",
          "base-100": "#ffffff",
        },
      },
    ],
  },
};
