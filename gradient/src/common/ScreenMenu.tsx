import styled from 'styled-components';

export type Screen =
  | 'delivery'
  | 'pickup'
  | 'config'
  | 'stats'
  | 'help'
  | 'admin';

type Props = {
  screen: Screen;
  setScreen: (screen: Screen) => void;
};
export const ScreenMenu = ({ screen, setScreen }: Props) => {
  return (
    <Container>
      <button
        disabled={screen === 'delivery'}
        onClick={() => setScreen('delivery')}
      >
        Delivery
      </button>
      <button
        disabled={screen === 'pickup'}
        onClick={() => setScreen('pickup')}
      >
        Pickup
      </button>
      <button
        disabled={screen === 'config'}
        onClick={() => setScreen('config')}
      >
        Config
      </button>
      <button disabled={screen === 'stats'} onClick={() => setScreen('stats')}>
        Stats
      </button>
      <button disabled={screen === 'help'} onClick={() => setScreen('help')}>
        Help
      </button>
    </Container>
  );
};

const Container = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 10px;
  margin-bottom: 0.5em;
`;
