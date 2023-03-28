import axios, { Axios, AxiosResponse } from 'axios';
import {
  Agent,
  Operation,
  Vec3,
  Item,
  Hold,
  Vec2,
  Dimension,
  Location,
  OperationStatus,
  PfResultNode
} from './types';

const BASE_URL = process.env.AGENT_ENDPOINT!;
const API_KEY = process.env.AGENT_API_KEY!;

const agentEndpoint = (name: string) => `${BASE_URL}/agent/${name}`;
const automationEndpoint = (name: string) => `${BASE_URL}/automation/${name}`;

const authHeader = { 'X-Api-Key': API_KEY };

const agentHeader = (agent: Agent) => ({
  'X-Agent-Id': agent.id,
  ...authHeader
});

export const registerAgent = async (): Promise<
  AxiosResponse<{
    agent: Agent;
  }>
> => axios.post(agentEndpoint('register'), undefined, { headers: authHeader });

export const heartbeat = async (agent: Agent): Promise<AxiosResponse<string>> =>
  axios.post(agentEndpoint('heartbeat'), undefined, {
    headers: agentHeader(agent)
  });

export const alert = async (
  description: string,
  agent: Agent
): Promise<AxiosResponse<string>> =>
  axios.post(
    agentEndpoint('alert'),
    { description },
    { headers: agentHeader(agent) }
  );

type PollOperationResponse =
  | {
      type: 'OperationUnavailable';
    }
  | {
      type: 'OperationAvailable';
      operation: Operation;
    };

export const pollOperation = async (
  agent: Agent,
  location: Location
): Promise<AxiosResponse<PollOperationResponse>> =>
  axios.post(
    agentEndpoint('poll_operation'),
    { location },
    {
      headers: agentHeader(agent)
    }
  );

export const operationComplete = async (
  agent: Agent,
  operation: Operation,
  finalStatus: OperationStatus
): Promise<AxiosResponse<string>> =>
  axios.post(
    agentEndpoint('operation_complete'),
    { operation_id: operation.id, final_status: finalStatus },
    { headers: agentHeader(agent) }
  );

export const inventoryScanned = async (
  slots: Array<Item | null>,
  inventoryLocation: Location,
  openFrom: Vec3,
  agent: Agent
): Promise<AxiosResponse<string>> =>
  axios.post(
    agentEndpoint('inventory_scanned'),
    {
      location: inventoryLocation,
      slots,
      open_from: openFrom
    },
    { headers: agentHeader(agent) }
  );

export const getHold = async (
  id: string,
  agent: Agent
): Promise<AxiosResponse<{ hold: Hold }>> =>
  axios.get(agentEndpoint(`hold/${id}`), { headers: agentHeader(agent) });

type FreeHoldResponse =
  | {
      type: 'HoldAcquired';
      hold: Hold;
    }
  | {
      type: 'HoldUnavailable';
    };

export const getFreeHold = async (
  agent: Agent
): Promise<AxiosResponse<FreeHoldResponse>> =>
  axios.post(agentEndpoint('hold/free'), undefined, {
    headers: agentHeader(agent)
  });

export const releaseHold = async (hold_id: string) =>
  axios.delete(automationEndpoint(`holds/${hold_id}`), {
    headers: authHeader
  });

type PathfindingResponse =
  | {
      type: 'PathFound';
      path: PfResultNode[];
    }
  | {
      type: 'Error';
    };

export const findPath = async (
  agent: Agent,
  startLoc: Location,
  endLoc: Location
): Promise<AxiosResponse<PathfindingResponse>> =>
  axios.post(
    agentEndpoint('pathfinding'),
    {
      start_loc: startLoc,
      end_loc: endLoc
    },
    {
      headers: agentHeader(agent)
    }
  );

export type Sign = {
  lines: string[];
  location: Location;
};

export type ScanRegion = {
  signs: Sign[];
  bounds: [Vec2, Vec2];
  dimension: Dimension;
};

export const sendSignScanData = (
  agent: Agent,
  scanRegions: ScanRegion[]
): Promise<AxiosResponse<string>> =>
  axios.post(
    agentEndpoint('sign_scan_data'),
    { scan_regions: scanRegions },
    { headers: agentHeader(agent) }
  );

export type PathfindingNode = {
  location: Location;
  name: string;
  connections: string[];
  pickup?: Vec3;
  dropoff?: Vec3;
};

export type StorageComplex = {
  FlatFloor: {
    dimension: Dimension;
    y_level: number;
    bounds: [Vec2, Vec2];
    name: string;
  };
  Tower: {
    dimension: Dimension;
    origin: Vec3;
    height: number;
    name: string;
  };
};

export type CompiledSignConfig = {
  nodes: { [name: string]: PathfindingNode };
  complexes: { [name: string]: StorageComplex };
};

export const getSignConfig = (): Promise<AxiosResponse<CompiledSignConfig>> =>
  axios.get(automationEndpoint('sign_config'), { headers: authHeader });
