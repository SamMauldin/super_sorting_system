import styled from 'styled-components';
import { ActionController, ActionStatus } from '.';

type Props = {
  actionController: ActionController;
};

const friendlyStatusString = (status: ActionStatus): string => {
  if (status === 'complete') {
    return 'Complete';
  } else if (status === 'in-progress') {
    return 'In Progress';
  } else if (status === 'failed') {
    return 'Failed';
  }

  throw new Error('Unknown action status!');
};

export const ActionStatusDisplay = ({ actionController }: Props) => {
  if (actionController.currentActions.length === 0) return <span />;

  return (
    <ActionStatusList>
      {actionController.currentActions.map((action) => {
        if (action.type === 'delivery') {
          return (
            <ActionStatusItem key={action.id}>
              Delivery to {action.node}: {friendlyStatusString(action.status)}
            </ActionStatusItem>
          );
        } else if (action.type === 'pickup') {
          return (
            <ActionStatusItem key={action.id}>
              Pickup from {action.node}: {friendlyStatusString(action.status)}
            </ActionStatusItem>
          );
        } else {
          return null;
        }
      })}
    </ActionStatusList>
  );
};

const ActionStatusList = styled.ul`
  list-style: none;
`;

const ActionStatusItem = styled.li``;
