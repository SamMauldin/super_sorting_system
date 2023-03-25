import { useEffect, useState } from 'react';
import styled from 'styled-components';

import { Delivery, Pickup, Config, Stats, Help } from './screens';
import shulker from './assets/shulker.png';
import {
  ActionStatusDisplay,
  Screen,
  ScreenMenu,
  useActionController,
} from './common';
import { Admin } from './screens/Admin';

export const App = () => {
  const [currentScreen, setCurrentScreen] = useState<Screen>('delivery');
  const actionController = useActionController();

  const goHome = () => setCurrentScreen('delivery');

  useEffect(() => {
    const handler = (ev: KeyboardEvent) => {
      if (ev.ctrlKey) {
        if (ev.key === 'd') {
          setCurrentScreen('delivery');
          ev.preventDefault();
        } else if (ev.key === 'p') {
          setCurrentScreen('pickup');
          ev.preventDefault();
        } else if (ev.key === 'c') {
          setCurrentScreen('config');
          ev.preventDefault();
        } else if (ev.key === 's') {
          setCurrentScreen('stats');
          ev.preventDefault();
        } else if (ev.key === 'h') {
          setCurrentScreen('help');
          ev.preventDefault();
        } else if (ev.key === 'a') {
          setCurrentScreen('admin');
          ev.preventDefault();
        }
      }
    };

    document.addEventListener('keydown', handler);

    return () => document.removeEventListener('keydown', handler);
  });

  return (
    <Container>
      <TitleBar>
        <Logo src={shulker} />
        <Title>Super Sorting System</Title>
      </TitleBar>

      <ScreenMenu screen={currentScreen} setScreen={setCurrentScreen} />

      <ActionStatusDisplay actionController={actionController} />

      {currentScreen === 'delivery' && (
        <Delivery actionController={actionController} />
      )}
      {currentScreen === 'pickup' && (
        <Pickup actionController={actionController} finished={goHome} />
      )}
      {currentScreen === 'config' && <Config />}
      {currentScreen === 'stats' && <Stats />}
      {currentScreen === 'help' && <Help />}
      {currentScreen === 'admin' && (
        <Admin actionController={actionController} />
      )}
    </Container>
  );
};

const Container = styled.div`
  display: grid;
  grid-template-rows: auto auto auto 1fr;
  grid-template-columns: 1fr min(100vw, 800px) 1fr;
  position: absolute;
  height: 100%;
  width: 100%;

  & > * {
    grid-column: 2;
  }
`;

const TitleBar = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 20px 10px;
`;

const Title = styled.h1`
  margin: 0px 0px 0px 10px;
`;

const Logo = styled.img`
  height: 50px;
`;
