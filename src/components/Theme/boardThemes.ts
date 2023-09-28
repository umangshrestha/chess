const commonTheme = "h-square w-square flex justify-center items-center";

export const boardThemes = {
  green: {
    light: `${commonTheme} bg-green-light`,
    dark: `${commonTheme} bg-green-dark`,
  },
  blue: {
    light: `${commonTheme} bg-blue-light`,
    dark: `${commonTheme} bg-blue-dark`,
  },
  pink: {
    light: `${commonTheme} bg-pink-light`,
    dark: `${commonTheme} bg-pink-dark`,
  },
  beige: {
    light: `${commonTheme} bg-beige-light`,
    dark: `${commonTheme} bg-beige-dark`,
  },
  purple: {
    light: `${commonTheme} bg-purple-light`,
    dark: `${commonTheme} bg-purple-dark`,
  },
  retro: {
    light: `${commonTheme} bg-retro-light`,
    dark: `${commonTheme} bg-retro-dark`,
  },
} as const;

export type BoardThemeType = keyof typeof boardThemes;
