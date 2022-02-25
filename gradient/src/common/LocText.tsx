import React from "react";
import { Dimension, Loc, Vec3 } from "../api/types";

const dimText: { [key in Dimension]: string } = {
  TheNether: "The Nether",
  TheEnd: "The End",
  Overworld: "The Overworld",
};

type Props = {
  location: Loc | Vec3;
};

export const LocText = ({ location }: Props) => {
  if ("dim" in location) {
    return (
      <span>
        X: {location.vec3.x}, Y: {location.vec3.y}, Z: {location.vec3.z} in{" "}
        {dimText[location.dim]}
      </span>
    );
  } else {
    return (
      <span>
        X: {location.x}, Y: {location.y}, Z: {location.z}
      </span>
    );
  }
};
