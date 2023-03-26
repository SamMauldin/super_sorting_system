import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';

import { getHold } from '../controllerApi';
import { Agent, Location, locEq, MoveItemsOperationKind, Vec3 } from '../types';
import { sendChestData, transferItems } from './procedures';
import { openChestAt } from './procedures/openChestAt';

export const moveItems = async (
  operationKind: MoveItemsOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const sourceHolds = (
    await Promise.all(
      operationKind.source_holds.map((hold) => getHold(hold, agent))
    )
  ).map((res) => res.data.hold);
  const destinationHolds = (
    await Promise.all(
      operationKind.destination_holds.map((hold) => getHold(hold, agent))
    )
  ).map((res) => res.data.hold);

  let lastChest: {
    location: Location;
    chest: Chest & Window;
    openFrom: Vec3;
  } | null = null;

  for (const [idx, hold] of sourceHolds.entries()) {
    if (lastChest && !locEq(hold.location, lastChest.location)) {
      await sendChestData(
        lastChest.chest,
        lastChest.location,
        lastChest.openFrom,
        agent
      );
      lastChest.chest.close();
      lastChest = null;
    }

    const chest: Chest & Window =
      lastChest?.chest ||
      (await openChestAt(hold.location, hold.open_from, bot, agent));

    await transferItems(
      bot,
      chest,
      hold.slot,
      idx,
      operationKind.counts[idx],
      'from_chest'
    );

    lastChest = {
      chest,
      location: hold.location,
      openFrom: hold.open_from
    };
  }

  for (const [idx, hold] of destinationHolds.entries()) {
    if (lastChest && !locEq(hold.location, lastChest.location)) {
      await sendChestData(
        lastChest.chest,
        lastChest.location,
        lastChest.openFrom,
        agent
      );
      lastChest.chest.close();
      lastChest = null;
    }

    const chest: Chest & Window =
      lastChest?.chest ||
      (await openChestAt(hold.location, hold.open_from, bot, agent));

    await transferItems(
      bot,
      chest,
      hold.slot,
      idx,
      operationKind.counts[idx],
      'to_chest'
    );

    lastChest = {
      chest,
      location: hold.location,
      openFrom: hold.open_from
    };
  }

  if (lastChest) {
    await sendChestData(
      lastChest.chest,
      lastChest.location,
      lastChest.openFrom,
      agent
    );
    lastChest.chest.close();
  }
};
