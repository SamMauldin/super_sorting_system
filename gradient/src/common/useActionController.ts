import { useRef, useState } from 'react';
import {
  deliverItems as apiDeliverItems,
  DeliveryItems,
} from '../api/delivery';
import { pickupItems as apiPickupItems } from '../api/pickup';
import { complexTransfer as apiComplexTransfer } from '../api/complex_transfer';

export type ActionStatus = 'in-progress' | 'complete' | 'failed';
export type ActionDetails = {
  id: number;
  status: ActionStatus;
};

export type DeliveryAction = ActionDetails & {
  type: 'delivery';
  node: string;
};
export type PickupAction = ActionDetails & {
  type: 'pickup';
  node: string;
};
export type ComplexTransfer = ActionDetails & {
  type: 'complex_transfer';
  from_complex: string;
  to_complex: string;
};

export type Action = DeliveryAction | PickupAction | ComplexTransfer;

export type ActionController = {
  currentActions: Action[];
  deliverItems: (node: string, items: DeliveryItems) => void;
  pickupItems: (node: string) => void;
  complexTransfer: (from_complex: string, to_complex: string) => void;
};

export const useActionController = (): ActionController => {
  const nextActionId = useRef(0);
  const getNextActionId = () => nextActionId.current++;

  const [currentActions, setCurrentActions] = useState<Action[]>([]);

  const finishAction = (actionId: number, status: ActionStatus) => {
    setCurrentActions((actions) => {
      const newActions = [...actions];
      const actionIdx = newActions.findIndex(
        (action) => action.id === actionId,
      );

      newActions[actionIdx] = {
        ...actions[actionIdx],
        status,
      };

      return newActions;
    });

    setTimeout(() => {
      setCurrentActions((actions) =>
        actions.filter((action) => action.id !== actionId),
      );
    }, 1000 * 15);
  };

  const deliverItems = (node: string, items: DeliveryItems) => {
    const actionId = getNextActionId();
    setCurrentActions((actions) => [
      ...actions,
      {
        id: actionId,
        status: 'in-progress',
        type: 'delivery',
        node,
      },
    ]);

    apiDeliverItems(node, items)
      .then(() => finishAction(actionId, 'complete'))
      .catch(() => finishAction(actionId, 'failed'));
  };

  const pickupItems = (node: string) => {
    const actionId = getNextActionId();
    setCurrentActions((actions) => [
      ...actions,
      {
        id: actionId,
        status: 'in-progress',
        type: 'pickup',
        node,
      },
    ]);

    apiPickupItems(node)
      .then(() => finishAction(actionId, 'complete'))
      .catch(() => finishAction(actionId, 'failed'));
  };

  const complexTransfer = (fromComplexName: string, toComplexName: string) => {
    const actionId = getNextActionId();
    setCurrentActions((actions) => [
      ...actions,
      {
        id: actionId,
        status: 'in-progress',
        type: 'complex_transfer',
        from_complex: fromComplexName,
        to_complex: toComplexName,
      },
    ]);

    apiComplexTransfer(fromComplexName, toComplexName);
  };

  return { currentActions, deliverItems, pickupItems, complexTransfer };
};
