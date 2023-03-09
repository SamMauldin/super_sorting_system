import { DefaultTheme } from 'styled-components';

export const theme: DefaultTheme = {
	bg0: "#282828",
	bg1: "#3c3836",
	bg2: "#504945",
	bg3: "#665c54",
	bg4: "#7c6f64",
	fg0: "#fbf1c7",
	fg1: "#ebdbb2",
	fg2: "#d5c4a1",
	fg3: "#bdae93",
	fg4: "#a89984",
	gray: "#a89984",

	red: "#fb4934",
	green: "#b8bb26",
	yellow: "#fabd2f",
	blue: "#83a598",
	purple: "#d3869b",
	aqua: "#8ec07c",
	orange: "#fe8019",
};

declare module 'styled-components' {
  export interface DefaultTheme {
    bg0: string;
    bg1: string;
    bg2: string;
    bg3: string;
    bg4: string;
    fg0: string;
    fg1: string;
    fg2: string;
    fg3: string;
    fg4: string;
    gray: string;

    red: string;
    green: string;
    yellow: string;
    blue: string;
    purple: string;
    aqua: string;
    orange: string;
  }
}
