/** @type {import("tailwindcss").Config} */

const size = {
  board: "600px",
  square: "75px",
  themeSelector: "55px",
  label: "20px",
  boardWithLabel: "600px",
};

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      width: size,
      height: size,
      colors: {
        green: {
          light: "#c5d6a9",
          dark: "#739957",
        },
        brown: {
          light: "#f0d9b5",
          dark: "#b58863",
        },
        blue: {
          light: "#d9e2f0",
          dark: "#7c9ac1",
        },
        pink: {
          light: "#f0d9e2",
          dark: "#c17c8e",
        },
        purple: {
          light: "#e2d9f0",
          dark: "#8c7cc1",
        },
        beige: {
          light: "#f0e9d9",
          dark: "#c1b17c",
        },
        retro: {
          light: "#e2e2e2",
          dark: "#8c8c8c",
        },
      },
    },
  },
};
