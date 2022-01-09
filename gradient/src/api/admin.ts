import axios, { AxiosResponse } from "axios";
import { Stats } from "./admin_types";

const BASE_URL = process.env.REACT_APP_API_BASE_URL!;
const API_KEY = process.env.REACT_APP_API_KEY_ADMIN!;

const endpoint = (name: string) => `${BASE_URL}/admin/${name}`;
const headers = { "X-Api-Key": API_KEY };

export const getStats = (): Promise<AxiosResponse<Stats>> =>
  axios.get(endpoint("stats"), { headers });
