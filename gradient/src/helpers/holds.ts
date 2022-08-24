import assert from 'assert';
import { createHold, removeHold, renewHold } from '../api/automation';
import { Hold } from '../api/types';
import { HoldRequestFilter } from '../api/automation_types';

export const acquireFreeSpaces = async (count: number): Promise<Hold[]> => {
  const filters = Array<HoldRequestFilter>(count).fill('EmptySlot');
  const res = await createHold(filters);

  const located = res.data.results.map((holds) => {
    if ('Error' in holds) {
      console.error(holds.Error.error);
      throw new Error('Failed to acquire empty slots!');
    }

    return holds.Holds.holds[0];
  });

  return located;
};

export const renewHolds = async (holds: string[]): Promise<void> => {
  for (const hold_id of holds) {
    const { data } = await renewHold(hold_id);
    assert(data.type === 'HoldRenewed', 'Unable to renew hold');
  }
};

export const releaseHolds = async (holds: string[]): Promise<void> => {
  for (const hold_id of holds) {
    await removeHold(hold_id);
  }
};
