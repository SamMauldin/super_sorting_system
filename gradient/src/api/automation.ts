import axios, { AxiosResponse } from "axios";
import {
  Hold,
  InventoriesWithLoc,
  OperationKind,
  Operation,
  OperationPriority,
  PathfindingNode,
} from "./automation_types";
import { Vec3 } from "./types";

const BASE_URL = process.env.REACT_APP_API_BASE_URL!;
const API_KEY = process.env.REACT_APP_API_KEY_AUTOMATION!;

const endpoint = (name: string) => `${BASE_URL}/automation/${name}`;
const headers = { "X-Api-Key": API_KEY };

export const getInventoryContents = (): Promise<
  AxiosResponse<InventoriesWithLoc>
> => axios.get(endpoint("inventory_contents"), { headers });

export const getPathfindingConfig = (): Promise<
  AxiosResponse<{ nodes: { [node_id: string]: PathfindingNode } }>
> => axios.get(endpoint("pathfinding_config"), { headers });

export const getHolds = (): Promise<AxiosResponse<{ holds: Hold[] }>> =>
  axios.get(endpoint("holds"), { headers });

type CreateHoldResponse =
  | {
      type: "HoldCreated";
      hold: Hold;
    }
  | {
      type: "Error";
    };

export const createHold = (
  location: Vec3,
  slot: number
): Promise<AxiosResponse<CreateHoldResponse>> =>
  axios.post(endpoint("holds"), { location, slot }, { headers });

type RemoveHoldResponse =
  | { type: "HoldRemoved"; hold: Hold }
  | { type: "HoldNotFound" };

export const removeHold = (
  hold_id: string
): Promise<AxiosResponse<RemoveHoldResponse>> =>
  axios.delete(endpoint(`holds/${hold_id}`), { headers });

type RenewHoldResponse =
  | { type: "HoldRenewed"; hold: Hold }
  | { type: "HoldNotFound" };

export const renewHold = (
  hold_id: string
): Promise<AxiosResponse<RenewHoldResponse>> =>
  axios.post(endpoint(`holds/${hold_id}/renew`), { headers });

export const createOperation = (
  kind: OperationKind,
  priority: OperationPriority
): Promise<AxiosResponse<{ operation: Operation }>> =>
  axios.post(endpoint("operations"), { kind, priority }, { headers });

export const getOperation = (
  operation_id: string
): Promise<AxiosResponse<{ operation: Operation }>> =>
  axios.get(endpoint(`operations/${operation_id}`), { headers });
