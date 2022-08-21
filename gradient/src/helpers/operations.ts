import { createOperation, getOperation } from '../api/automation';
import { OperationKind, OperationPriority } from '../api/automation_types';
import { delay } from '.';

export const pollOperation = async (operation_id: string): Promise<void> => {
  while (true) {
    const {
      data: { operation },
    } = await getOperation(operation_id);

    if (operation.status === 'Complete') return;
    if (operation.status === 'Aborted') throw new Error('Operation aborted!');

    await delay(1000);
  }
};

export const executeOperation = async (
  kind: OperationKind,
  priority: OperationPriority,
) => {
  const {
    data: { operation },
  } = await createOperation(kind, priority);

  await pollOperation(operation.id);
};
