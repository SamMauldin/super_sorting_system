import axios, { AxiosResponse } from 'axios';
import {
  Hold,
  InventoriesWithLoc,
  OperationKind,
  Operation,
  OperationPriority,
  CompiledSignConfig,
  HoldMatchResult,
  HoldRequestFilter,
} from './automation_types';
import { Item } from './types';

const BASE_URL = process.env.REACT_APP_API_BASE_URL!;
const API_KEY = process.env.REACT_APP_API_KEY!;

const endpoint = (name: string) => `${BASE_URL}/automation/${name}`;
const headers = { 'X-Api-Key': API_KEY };

export const getInventoryContents = (): Promise<
  AxiosResponse<InventoriesWithLoc>
> => axios.get(endpoint('inventory_contents'), { headers });

export const getInventoryListing = (): Promise<AxiosResponse<Array<Item>>> =>
  axios.get(endpoint('inventory_listing'), { headers });

export const getSignConfig = (): Promise<AxiosResponse<CompiledSignConfig>> =>
  axios.get(endpoint('sign_config'), { headers });

export const getHolds = (): Promise<AxiosResponse<{ holds: Hold[] }>> =>
  axios.get(endpoint('holds'), { headers });

type CreateHoldResponse = {
  results: HoldMatchResult[];
};

export const createHold = (
  requests: HoldRequestFilter[],
): Promise<AxiosResponse<CreateHoldResponse>> =>
  axios.post(endpoint('holds'), { requests }, { headers });

type RemoveHoldResponse =
  | { type: 'HoldRemoved'; hold: Hold }
  | { type: 'HoldNotFound' };

export const removeHold = (
  hold_id: string,
): Promise<AxiosResponse<RemoveHoldResponse>> =>
  axios.delete(endpoint(`holds/${hold_id}`), { headers });

export const createOperation = (
  kind: OperationKind,
  priority: OperationPriority,
): Promise<AxiosResponse<{ operation: Operation }>> =>
  axios.post(endpoint('operations'), { kind, priority }, { headers });

export const getOperation = (
  operation_id: string,
): Promise<AxiosResponse<{ operation: Operation }>> =>
  axios.get(endpoint(`operations/${operation_id}`), { headers });
