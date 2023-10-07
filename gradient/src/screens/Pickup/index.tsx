import { useState } from 'react';
import { ActionController, NodeSelector } from '../../common';

type Props = {
  actionController: ActionController;
  finished: () => void;
};

export const Pickup = ({ actionController, finished }: Props) => {
  const [repeatUntilEmpty, setRepeatUntilEmpty] = useState<boolean>(false);

  const submit = (node: string) => {
    actionController.pickupItems(node, repeatUntilEmpty);
    finished();
  };

  return (
    <div>
      <h2>Pickup</h2>
      <div>
        <input
          type="checkbox"
          id="continual-pickup"
          checked={repeatUntilEmpty}
          onChange={({ target: { checked } }) => {
            setRepeatUntilEmpty(checked);
          }}
        />
        <label htmlFor="continual-pickup">Repeat Until Empty</label>
      </div>
      <NodeSelector submit={submit} purpose="pickup" />
    </div>
  );
};
