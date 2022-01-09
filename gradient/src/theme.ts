import { DefaultTheme } from "styled-components";

export const theme: DefaultTheme = {
  grey: "#333533",
  blue: "#388697",
  yellow: "#fffd77",
  orange: "#fa8334",
};

declare module "styled-components" {
  export interface DefaultTheme {
    grey: string;
    blue: string;
    yellow: string;
    orange: string;
  }
}
