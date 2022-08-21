import styled from 'styled-components';
import { ExtendedItem } from '../helpers';
import { KeyboardEvent, useState } from 'react';

type Props = {
  item: ExtendedItem;
  startingCount: number;
  close: (selectedCount: number | null) => void;
};

export const CountSelectorModal = ({ startingCount, item, close }: Props) => {
  const [count, setCount] = useState(startingCount);

  const clampValue = (count: number) =>
    Math.max(0, Math.min(count, item.count));

  const onClose = () => close(count);

  const onKeyDown = (ev: KeyboardEvent) => {
    const increment = ev.shiftKey ? item.stack_size : 1;

    if (ev.key === 'Enter') {
      onClose();
    } else if (ev.key === 'Escape') {
      close(null);
    } else if (ev.key === 'ArrowUp' || ev.key === 'Up') {
      ev.preventDefault();

      setCount(clampValue(count + increment));
    } else if (ev.key === 'ArrowDown' || ev.key === 'Down') {
      ev.preventDefault();

      setCount(clampValue(count - increment));
    }
  };

  const stacks = Math.floor(count / item.stack_size);
  const finalStackSize = count % item.stack_size;

  return (
    <Container>
      <p>{item.prettyPrinted}</p>
      <input
        autoFocus
        type="number"
        min={0}
        max={item.count}
        value={count}
        onChange={({ target: { valueAsNumber } }) =>
          setCount(clampValue(isNaN(valueAsNumber) ? 0 : valueAsNumber))
        }
        onKeyDown={onKeyDown}
      />
      <p>
        {stacks} stacks + {finalStackSize} items
      </p>
      <button type="submit" onClick={onClose}>
        Select
      </button>
    </Container>
  );
};

const Container = styled.div`
  padding: 10px;
  background-color: white;
  color: black;
  border-radius: 5px;

  display: flex;
  flex-direction: column;

  & > * {
    margin: 10px;
  }
`;
