import styled from 'styled-components';
import { ExtendedItem } from '../helpers';
import { KeyboardEvent, useState } from 'react';

type Denomination = 'item' | 'stack' | 'shulker';
type Props = {
  item: ExtendedItem;
  startingCount?: number;
  close: (selectedCount: number | null) => void;
};

const formatShort = (num: number): string => {
  const rounded = Math.floor(num * 10) / 10;
  return String(rounded);
};

export const CountSelectorModal = ({ startingCount, item, close }: Props) => {
  const [rawCount, setRawCount] = useState(startingCount ? startingCount : 1);
  const [denomination, setDenomination] = useState<Denomination>(
    startingCount ? 'item' : 'stack',
  );

  const stackMult = item.stack_size;
  const shulkerMult = stackMult * 27;

  const applyDenomination = (count: number) => {
    if (denomination === 'item') return count;
    if (denomination === 'shulker') return count * shulkerMult;
    return count * stackMult;
  };

  const appliedCount = applyDenomination(rawCount);

  const clampValue = (count: number) =>
    Math.max(0, Math.min(Math.round(count), item.count));

  const onClose = () => close(appliedCount);

  const onKeyDown = (ev: KeyboardEvent) => {
    if (ev.key === 'Enter') {
      onClose();
    } else if (ev.key === 'Escape') {
      close(null);
    } else if (ev.key === 'ArrowUp' || ev.key === 'Up') {
      ev.preventDefault();
      setRawCount(rawCount + 1);
    } else if (ev.key === 'ArrowDown' || ev.key === 'Down') {
      ev.preventDefault();
      setRawCount(rawCount - 1);
    } else if (ev.key === 'Tab') {
      ev.preventDefault();
      setDenomination((denom) => {
        if (ev.shiftKey) {
          if (denom === 'shulker') return 'stack';
          if (denom === 'stack') return 'item';
          return 'shulker';
        }

        if (denom === 'shulker') return 'item';
        if (denom === 'stack') return 'shulker';
        return 'stack';
      });
    }
  };

  const shulkers = Math.floor(appliedCount / shulkerMult);
  const remainingAfterShulkers = appliedCount - shulkers * shulkerMult;
  const stacks = Math.floor(remainingAfterShulkers / stackMult);
  const remainingAfterStacks = remainingAfterShulkers - stacks * stackMult;

  return (
    <Container>
      <p>{item.prettyPrinted}</p>
      <InputRow>
        <input
          autoFocus
          type="number"
          min={0}
          max={item.count}
          value={rawCount}
          onChange={({ target: { valueAsNumber } }) =>
            setRawCount(clampValue(isNaN(valueAsNumber) ? 0 : valueAsNumber))
          }
          onKeyDown={onKeyDown}
        />
        <span>
          {denomination === 'item' && 'Items'}
          {denomination === 'stack' && 'Stacks'}
          {denomination === 'shulker' && 'Shulkers'}
        </span>
      </InputRow>

      <p>
        {shulkers} shulkers + {stacks} stacks + {remainingAfterStacks} items
      </p>
      <p>
        ({formatShort(appliedCount / shulkerMult)} shulkers /{' '}
        {formatShort(appliedCount / stackMult)} stacks / {appliedCount} items)
      </p>

      <button type="submit" onClick={onClose}>
        Select
      </button>
    </Container>
  );
};

const Container = styled.div`
  padding: 10px;
  background-color: ${({ theme }) => theme.fg0};
  color: black;
  border-radius: 5px;

  display: flex;
  flex-direction: column;

  width: 400px;

  & > * {
    margin: 10px;
  }
`;

const InputRow = styled.div`
  display: flex;
  gap: 10px;
`;
