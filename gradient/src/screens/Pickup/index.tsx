import { ActionController, NodeSelector } from '../../common';

type Props = {
  actionController: ActionController;
  finished: () => void;
};

export const Pickup = ({ actionController, finished }: Props) => {
  const submit = (node: string) => {
    actionController.pickupItems(node);
    finished();
  };

  return (
    <div>
      <h2>Pickup</h2>
      <NodeSelector submit={submit} purpose="pickup" />
    </div>
  );
};
