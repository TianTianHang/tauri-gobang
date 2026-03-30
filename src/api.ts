import type { AuthResponse, RoomListEntry, CreateRoomResponse, JoinRoomResponse } from "./types/game";

const DEFAULT_SERVER_URL = "http://localhost:3001";

function getServerUrl(): string {
  return localStorage.getItem("gobang_server_url") || DEFAULT_SERVER_URL;
}

function wsBaseUrl(): string {
  const url = getServerUrl();
  return url.replace(/^http/, "ws");
}

function apiUrl(path: string): string {
  return `${getServerUrl()}${path}`;
}

async function request<T>(url: string, options?: RequestInit, token?: string): Promise<T> {
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...options?.headers,
  };
  if (token) {
    (headers as Record<string, string>)["Authorization"] = `Bearer ${token}`;
  }
  const res = await fetch(url, { ...options, headers });
  const data = await res.json();
  if (!res.ok) {
    throw new Error(data.error || `请求失败 (${res.status})`);
  }
  return data as T;
}

export async function register(username: string, password: string): Promise<AuthResponse> {
  return request<AuthResponse>(apiUrl("/api/register"), {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

export async function login(username: string, password: string): Promise<AuthResponse> {
  return request<AuthResponse>(apiUrl("/api/login"), {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

export async function getRooms(token: string): Promise<{ rooms: RoomListEntry[] }> {
  return request<{ rooms: RoomListEntry[] }>(apiUrl("/api/rooms"), undefined, token);
}

export async function createRoom(token: string, name: string): Promise<CreateRoomResponse> {
  return request<CreateRoomResponse>(apiUrl("/api/rooms"), {
    method: "POST",
    body: JSON.stringify({ name }),
  }, token);
}

export async function joinRoom(token: string, roomId: string): Promise<JoinRoomResponse> {
  return request<JoinRoomResponse>(apiUrl(`/api/rooms/${roomId}/join`), {
    method: "POST",
  }, token);
}

export function getGameWsUrl(roomId: string, token: string): string {
  return `${wsBaseUrl()}/game/${roomId}?token=${encodeURIComponent(token)}`;
}

export function setServerUrl(url: string): void {
  localStorage.setItem("gobang_server_url", url);
}

export function getStoredServerUrl(): string {
  return getServerUrl();
}
