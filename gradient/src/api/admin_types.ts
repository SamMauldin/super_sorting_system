export type Stats = {
  inventories_in_mem: number;
  total_slots: number;
  free_slots: number;

  current_holds: number;

  operations_pending: number;
  operations_in_progress: number;
  operations_complete: number;
  operations_aborted: number;

  agents_connected: number;
};
