import { useEffect, useState } from "react";
import styled from "styled-components";

import { Delivery, Pickup, Stats, Help } from "./screens";
import shulker from "./assets/shulker.png";

export const App = () => {
  const [currentLocation, setCurrentLocation] = useState<
    "delivery" | "pickup" | "stats" | "help"
  >("delivery");

  useEffect(() => {
    const handler = (ev: KeyboardEvent) => {
      if (!ev.ctrlKey && !ev.altKey) return;

      if (ev.key === "d") {
        setCurrentLocation("delivery");
        ev.preventDefault();
      } else if (ev.key === "p") {
        setCurrentLocation("pickup");
        ev.preventDefault();
      } else if (ev.key === "s") {
        setCurrentLocation("stats");
        ev.preventDefault();
      } else if (ev.key === "h") {
        setCurrentLocation("help");
        ev.preventDefault();
      }
    };

    document.addEventListener("keydown", handler);

    return () => document.removeEventListener("keydown", handler);
  }, []);

  return (
    <Container>
      <TitleBar>
        <Logo src={shulker} />
        <Title>Gradient</Title>
        <Button
          disabled={currentLocation === "delivery"}
          onClick={() => setCurrentLocation("delivery")}
        >
          Delivery
        </Button>
        <Button
          disabled={currentLocation === "pickup"}
          onClick={() => setCurrentLocation("pickup")}
        >
          Pickup
        </Button>
        <Button
          disabled={currentLocation === "stats"}
          onClick={() => setCurrentLocation("stats")}
        >
          Stats
        </Button>
        <Button
          disabled={currentLocation === "help"}
          onClick={() => setCurrentLocation("help")}
        >
          Help
        </Button>
      </TitleBar>
      {currentLocation === "delivery" && <Delivery />}
      {currentLocation === "pickup" && <Pickup />}
      {currentLocation === "stats" && <Stats />}
      {currentLocation === "help" && <Help />}
    </Container>
  );
};

const Container = styled.div`
  display: grid;
  height: 100%;
  width: 100%;
  position: absolute;
  grid-template-rows: min-content 1fr;
  grid-template-columns: 1fr;
`;

const TitleBar = styled.div`
  display: flex;
  align-items: center;
  margin: 2px 10px;
`;

const Title = styled.h1`
  margin: 0px 0px 0px 10px;
  font-size: px;
`;

const Logo = styled.img`
  height: 50px;
`;

const Button = styled.button`
  margin-left: 10px;
`;
