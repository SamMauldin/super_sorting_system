import { useState } from 'react';
import { useQuery } from 'react-query';
import styled from 'styled-components';
import { getSignConfig } from '../../api/automation';
import { ActionController } from '../../common';
import { SplashScreen } from '../SplashScreen';

type Props = {
  actionController: ActionController;
};

export const Admin = ({ actionController }: Props) => {
  const { error, isLoading, data } = useQuery('sign_config', getSignConfig, {
    refetchInterval: 1000 * 3,
  });

  const [fromComplex, setFromComplex] = useState<string | null>(null);
  const [toComplex, setToComplex] = useState<string | null>(null);

  const valid = Boolean(fromComplex && toComplex);

  if (error) return <SplashScreen message="Error loading stats!" />;
  if (isLoading || !data) return <SplashScreen message="Loading stats" />;

  const transfer = () =>
    fromComplex &&
    toComplex &&
    actionController.complexTransfer(fromComplex, toComplex);

  return (
    <Container>
      <h2>Admin Tools</h2>
      <h3>Complex Transfer</h3>
      <select
        value={fromComplex ?? ''}
        onChange={({ target: { value } }) => setFromComplex(value)}
      >
        <option value="">Transfer From</option>
        {Object.keys(data.data.complexes).map((complexName) => (
          <option key={complexName} value={complexName}>
            {complexName}
          </option>
        ))}
      </select>

      <select
        value={toComplex ?? ''}
        onChange={({ target: { value } }) => setToComplex(value)}
      >
        <option value="">Transfer To</option>
        {Object.keys(data.data.complexes).map((complexName) => (
          <option key={complexName} value={complexName}>
            {complexName}
          </option>
        ))}
      </select>

      <button disabled={!valid} onClick={transfer}>
        Transfer
      </button>
    </Container>
  );
};

const Container = styled.div`
  margin: 10px;
`;
