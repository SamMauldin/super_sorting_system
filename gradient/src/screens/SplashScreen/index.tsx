import { ReactNode } from 'react';
import styled from 'styled-components';
import shulker from '../../assets/shulker.png';

type Props = {
  message?: ReactNode;
};

export const SplashScreen = ({ message }: Props) => (
  <Container>
    <ShulkerImg src={shulker} alt="Shulker Box" />
    <MessageContainer>
      <h1>Super Sorting System</h1>
      {message && <p>{message}</p>}
    </MessageContainer>
  </Container>
);

const Container = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
`;

const ShulkerImg = styled.img`
  width: 15em;
`;

const MessageContainer = styled.div`
  margin-left: 5em;
`;
